#  Sample Smart contracts for Finschia

## Contracts
### delivery
- [`delivery-contract`](./contracts/delivery-contract) Simple implementation of logistics product(tea, food, etc.) traceability using dynamic link

### auction
- [`auction`](./contracts/auction/contracts/auction) Implementation of NFT auction system using dynamic link
- [`cw721-base-dynamiclink`](./contracts/auction/contracts/cw721-base-dynamiclink) cw721-base(nft) with callable points for dynamic link

## Compiling

```sh
$ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/contracts/{Contract}/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.9 ./contracts/{Contract}
```
This will compile all contracts in the ./contracts/{Contract} like ./contracts/auction directory and output the stripped and optimized wasm code under the artifacts directory as output, along with a checksums.txt file.

## Licenses

This repository is licensed under [Apache 2.0](./LICENSE)

All _specifications_ will always be Apache-2.0. All Sample contracts will also be Apache-2.0.