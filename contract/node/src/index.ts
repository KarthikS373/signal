import * as fs from "fs";

import dotenv from "dotenv";
import { Coin, SecretNetworkClient, Wallet } from "secretjs";

import { generateRandomString } from "../utils";

dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

// Load contract wasm
let contract_wasm: Buffer;
try {
  contract_wasm = fs.readFileSync("../contract.wasm.gz");
} catch (err) {
  console.log("Error loading contract wasm");
  console.log(err);
  process.exit(1);
}

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const deployContract = async () => {
  try {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contract_wasm,
        source: "",
        builder: "",
      },
      {
        gasLimit: 4_000_000,
      }
    );

    const codeId = Number(
      tx.arrayLog?.find(
        (log) => log.type === "message" && log.key === "code_id"
      )?.value
    );

    const contractCodeHash = (
      await secretjs.query.compute.codeHashByCodeId({
        code_id: codeId.toString(),
      })
    ).code_hash;
    console.log({ codeId, contractCodeHash });
  } catch (err) {
    console.log(err);
  }
};

const instantiateContract = async (
  codeId: string | number,
  contractCodeHash: string
) => {
  const initMsg = {
    entropy: generateRandomString(20),
    // TODO: Initial stake *should not be* zero, but is hardcoded for now to match the base stake
    // creator_base_stake: String(0),
    // validator_base_stake: String(0),
    creator_base_stake: String(15_000_000_000_000_000_000),
    validator_base_stake: String(8_000_000_000_000_000_000),
  };
  console.log(initMsg);
  let tx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "test__signal-" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );

  const contractAddress = tx.arrayLog?.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )?.value;

  console.log({ contractAddress });
};

const queryContract = async (
  contract_address: string,
  contractCodeHash: string,
  method: string
) => {
  let query_method = null;
  switch (method) {
    case "config":
      query_method = { get_config: {} };
      break;
    case "profile":
      query_method = { get_profile_with_viewing_key: {} };
      break;
    case "news":
      query_method = { get_all_news_items: {} };
      break;
    case "validations":
      query_method = { get_validations: {} };
      break;
    default:
      query_method = null;
  }

  if (query_method === null) {
    console.log("Invalid method");
    return;
  }

  try {
    console.log("Querying contract");

    const query = await secretjs.query.compute.queryContract({
      contract_address: contract_address,
      code_hash: contractCodeHash,
      query: query_method,
    });
    console.log("Query result:");
    console.log(query);
  } catch (err) {
    console.log("Error querying contract");
    console.log(err);
  }
};

const executeContract = async (
  contract_address: string,
  contractCodeHash: string,
  method: string
) => {
  let query_method = null,
    funds: Coin[] = [];

  switch (method) {
    case "create_creator":
      const viewing_key = generateRandomString(20);
      console.log("Viewing key: " + viewing_key);
      query_method = {
        create_creator_profile: {
          stake: "0", // Initial stake would be 0
          viewing_key: viewing_key, // Random viewing key
        },
      };
      break;
    case "create_validator":
      query_method = { create_validator_profile: {} };
      break;
    case "post_news":
      const contentHash = process.argv[6];
      console.log(`content IPFS hash: ${contentHash}`);
      if (!contentHash) throw new Error("Content IPFS hash is required");

      query_method = {
        post_news: {
          content: process.argv[6],
        },
      };
      break;
    case "validate_news":
      query_method = { validate_news: {} };
      break;
    default:
      query_method = null;
  }

  if (query_method === null) {
    console.log("Invalid method");
    return;
  }

  try {
    console.log("Executing contract");

    let tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: contractCodeHash,
        msg: query_method,
        sent_funds: funds,
      },
      {
        gasLimit: 100_000,
      }
    );
    console.log("Tx result:");
    console.log(tx);
  } catch (err) {
    console.log("Error executing contract");
    console.log(err);
  }
};

// Command-line interface
const command = process.argv[2];
console.log(`
Command: ${command}
Args: 
 - ${process.argv[3]} 
 - ${process.argv[4]}
 - ${process.argv[5]}
`);
switch (command) {
  case "deploy":
    deployContract();
    break;
  case "init":
    instantiateContract(
      process.argv[3], // codeId
      process.argv[4] // contractCodeHash
    );
    break;
  case "query":
    queryContract(
      process.argv[3], // contract_address
      process.argv[4], // contractCodeHash
      process.argv[5] // method
    );
    break;
  case "execute":
    executeContract(
      process.argv[3], // contract_address
      process.argv[4], // contractCodeHash
      process.argv[5] // method
    );
    break;
  default:
    console.log(
      "Invalid command. Usage: node index.js [deploy | init | query | execute]"
    );
}
