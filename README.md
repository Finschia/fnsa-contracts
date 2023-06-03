# dynamic_link_sample


## Optimized builds

```sh
$docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_delivery_contract",target=/code/delivery-contract/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.9 ./delivery-contract 
```