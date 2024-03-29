# map_deployments

A map module takes in blocks and makes a subgraph.
By looking at transaction traces we filter out:

- transactions that don't have a successful transaction status
- any call whose state is reverted
- any call type that is not Create

We create a variable `all_calls` that is assigned unfiltered calls for later use

### ERC20's

We pass in the current call being iterated on into a method on `ERC20Creation`
named `from_call` that checks if the call is from an ERC20.

If it is, the method returns a `ERC20Creation` struct that looks like this:

```rust
pub struct ERC20Creation {
    pub address: Vec<u8>,
    pub code: Vec<u8>,
    pub storage_changes: HashMap<H256, Vec<u8>>,
}
```

The `ERC20Creation` struct is then passed into the helper function
`process_erc20_contract` that uses `code` and `storage_changes` to simulate function execution using a local evm instance and populates a
`deployment` struct with the name, symbol, decimals, and total_supply.

We use the fields in the `deployment` struct to build the table in our subgraph.

### ERC721's

If the `from_call` on `ERC20Creation` detects the call is not an ERC20, the call
is passed into the `from_call` method on `ERC721Creation`.

Additionally, `all_calls` is also passed into `from_call` to detect proxy
contracts that could have made the ERC721 by analyzing the call tree.

`from_call` on `ERC721Creation` then returns this struct:

```rust
pub struct ERC721Creation {
    pub address: Vec<u8>,
    pub code: Vec<u8>,
    pub storage_changes: HashMap<H256, Vec<u8>>,
}
```

The `ERC721Creation` struct is then passed into the helper
`process_erc721_contract` that uses `code` and `storage_changes`  to simulate function execution using out local evm instance and populates a deployment struct with the name and symbol.

We use the fields in the `deployment` struct to build the table in our subgraph.
