import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
import { generateRandomString } from "../utils";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

let upload_contract = async () => {
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

// upload_contract();
// // {
// //   codeId: 5117,
// //   contractCodeHash: '33b31cc5ea704fdc04825c6c32e7bbe72292fa0756cf8d4ef5e35ef0a874d66c'
// // }

let instantiate_contract = async (
  codeId: string | number,
  contractCodeHash: string
) => {
  // Create an instance of the Counter contract, providing a starting count
  const initMsg = {
    entropy: generateRandomString(20),
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
      label: "My NewsBobchain" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );

  //Find the contract_address in the logs
  const contractAddress = tx.arrayLog?.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )?.value;

  console.log({ contractAddress });
};

// instantiate_contract(
//   5117,
//   "33b31cc5ea704fdc04825c6c32e7bbe72292fa0756cf8d4ef5e35ef0a874d66c"
// );
// // { contractAddress: 'secret1y08gl9u34h9g5vj72qhljxc9j95avg878q5sz2' }

let try_query_count = async (
  contract_address: string,
  contractCodeHash: string
) => {
  const my_query = await secretjs.query.compute.queryContract({
    contract_address: contract_address,
    code_hash: contractCodeHash,
    query: { get_count: {} },
  });

  console.log(my_query);
};

try_query_count(
  "secret1y08gl9u34h9g5vj72qhljxc9j95avg878q5sz2",
  "33b31cc5ea704fdc04825c6c32e7bbe72292fa0756cf8d4ef5e35ef0a874d66c"
);
// > { count: 0 }

let try_increment_count = async (
  contract_address: string,
  contractCodeHash: string
) => {
  console.log("incrementing...");
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash: contractCodeHash, // optional but way faster
      msg: {
        increment: {},
      },
      sent_funds: [], // optional
    },
    {
      gasLimit: 100_000,
    }
  );
  console.log("Done");
};

// try_increment_count(
//   "secret19mjewp9932vwrsj7lpnjlad5n4ua5nzhvty4yp",
//   "922efd1650f7d992ee16426b38e3cebb55c53a9812ecdbfa106ee36948c3d05c"
// );
