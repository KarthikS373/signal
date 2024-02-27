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
