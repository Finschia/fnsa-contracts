#  Sample Smart contracts for Finschia
* [contracts/delivery-contract](./contracts/delivery-contract): Simple implementation of logistics product(tea, food, etc.) traceability using dynamic link

## Optimized builds

```sh
$docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_delivery_contract",target=/code/delivery-contract/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.9 ./delivery-contract 
```

## Licenses

This repository is licensed under [Apache 2.0](./LICENSE)

All _specifications_ will always be Apache-2.0. All Sample contracts will also be Apache-2.0.