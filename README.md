#  Sample Smart contracts for Finschia
This repository provides sample contracts that use Finschia's dynamic link feature. Dynamic link is a new feature introduced in Finschia's cosmwasm that allows you to call functions of other contract directly from your contract code. For a detailed explanation of dynamic links, see [Links]().

This README explains how to use dynamic links and provides example code. This repository was created to provide a reference for contract developer to understand and utilize dynamic links.

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

cosmwasm/rust-optimizer currently has versions of both optimizers for two processor architectures: Intel/Amd 64-bits, and Arm 64-bits (these run natively on Mac M1 machines).

However, the native Arm version produces different wasm artifacts than the Intel version. Given that that impacts reproducibility, non-Intel images and build artifacts contain a "-arm64" suffix, to differentiate and flag them.

Arm images are released to ease development and testing on Mac M1 machines. For release / production use, only contracts built with the Intel optimizers must be used.

## Licenses

This repository is licensed under [Apache 2.0](./LICENSE)

All _specifications_ will always be Apache-2.0. All Sample contracts will also be Apache-2.0.