## Compile code to generate `contract.wasm`

Linux

```
make build
```

Windows

```
$env:RUSTFLAGS='-C link-arg=-s'
cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/*.wasm ./contract.wasm
```

## Optimize compiled wasm (for lower gas fees)

Linux

```
docker run --rm -v "$(pwd)":/contract \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer
```

Windows

```
docker run --rm -v "${PWD}:/contract" `
  --mount type=volume,source="$((Get-Item -Path $PWD).BaseName)_cache",target=/code/target `
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry `
  enigmampc/secret-contract-optimizer
```

## Deploy on SCRT network

`cd ./node`

**Make sure to place your wallet MNEMONIC first (see .env.example)**

`ts-node ./src/index.ts deploy`

> { codeId: 5175, contractCodeHash: '21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f' }

## Initialize contract

`ts-node ./src/index.ts init <codeId> <contractCodeHash>`

Example: `ts-node ./src/index.ts init 5175 21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f`

> { contractAddress: 'secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8' }

## Execute contract

`ts-node .\src\index.ts execute <contractAddress> <contractCodeHash> [create_creator | create_validator | post_news | validate_news]`

Example 1: `ts-node .\src\index.ts execute secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8 21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f create_creator`

Example 2: `ts-node .\src\index.ts execute secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8 21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f post_news abc14235`

## Query contract

`ts-node ./src/index.ts query <contractAddress> <contractCodeHash> [config | news | profile | validations]`

Example: `ts-node .\src\index.ts query secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8 21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f news`

```
Query result: [
  {
    id: '1',
    creator: '5ff458433270893da736b827486a65b152a1c86725a244e6e47fbf565d7f4bf0',
    content: 'abc14235'
  }
]
```
