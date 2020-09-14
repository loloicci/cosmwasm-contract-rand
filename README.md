# An Example of Non-deterministic Contract (using rand)

## Summary
Non-deterministic contracts are expected not to be able to stored in chain.
However, contracts using `rand::Rng.gen()` can be stored in the chain.

In the `master` branch, both init and execute are non-deterministic.
In the `deterministic-init` branch, only execute is non-deterministic.

Conclusion of comparing these, it issues a `out of gas` error when non-deterministic part (`rand::Rng.gen()`) is executed.

Why `out of gas` is issued is referred by https://github.com/CosmWasm/cosmwasm/issues/501 and https://github.com/CosmWasm/cosmwasm/issues/375 .

## Compile
This section is summary of https://docs.cosmwasm.com/getting-started/compile-contract.html .
For set up the environment, see https://docs.cosmwasm.com/getting-started/installation.html and https://docs.cosmwasm.com/getting-started/setting-env.html .

To compile this to wasm suitable to use in CosmWasm, execute the follows.

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.9.0
```

Then, `contract.wasm` is the output.


## Execute
I installed and executed the contract in the local chain following https://docs.cosmwasm.com/getting-started/interact-with-contract.html .
Notice there is a bug. see also https://github.com/CosmWasm/docs2/issues/26
