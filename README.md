# map_blocks

We create two vectors `deployments` and `mints` to collect filtered data.

Their purpose is to be passed into a helper that implements the `evm_core` to
retrieve the token URI.

By looking at transaction traces we filter out:

- transactions that don't have a successful transaction status
- any call whose state is reverted

---

We create a variable `all_calls` that is assigned unfiltered calls for later use.

### Handling Mints

We iterate over the `logs` of the current `call`.

For each `log` of that `call`, we verify the log is an `ERC721TransferEvent`.

We also verify the `transfer.from` address is `NULL` since token
minting always have a `NULL` from address.

Then we populate a protobuf called `ERC721Mint` and push it into our `mints`
vector like this:

```rust
mints.push(Erc721Mint {
    token_id: transfer.token_id.to_string(),
    address: Hex::encode(&log.address),
    blocknumber: blk.number,
    timestamp_seconds: blk.timestamp_seconds(),
});
```

### Handling Contract Deployments

While iterating over the calls to collect `mints`, we also collect `deployments`.

We check if the `CallType` is `Create`.

If the `call` is of `CallType::Create`, the call is passed into the `from_call`
method on `ERC721Creation` struct.

Additionally, `all_calls` is also passed into `from_call` to detect proxy
contracts that could have made the ERC721.

`from_call` on `ERC721Creation` then returns this struct:

```rust
pub struct ERC721Creation<'a> {
    pub address: Vec<u8>,
    pub code: Vec<u8>,
    pub storage_changes: Vec<&'a StorageChange>,
}
```

The `ERC721Creation` struct is then passed into the helper
`process_erc721_contract` that uses `code` and `storage_changes` to create a
`deployment` struct.

The `deployment` struct is then pushed into our `deployments` vector.

`map_blocks` then creates a `MasterProto` protobuf with the `mints` and
`deployments` vector.

# store_contract_data

`store_contract_data` stores the `contracts` using the `StoreSetProto` store.

We use the `&contract.address` as the key and store the `&contract`.

We store this information so we can fetch the storage changes and bytecode when
we encounter mints as they happen.

# graph_out

The `graph_out` takes in `MasterProto` and `StoreGetProto`.

### NftToken

To fill out `NftToken` row we iterate over the `mints` in the `MasterProto` and
to retrieve the specific `contract`.

Once we have the `contract`, we pass it into the helper function `get_token_uri`
along with the `token_id` of the `mint` to get the `token_uri`.

Then we populate the table with the following:

```rust
tables
    .update_row("NftToken", format!("{}-{}", mint.address, mint.token_id))
    .set("tokenID", &mint.token_id)
    .set("tokenURI", token_uri)
    .set("address", &mint.address)
    .set("blocknumber", mint.blocknumber)
    .set("timestamp", mint.timestamp_seconds)
    .set("collection", &mint.address);
```

### NftDeployment

To fill out the `NftDeployment` row we iterate over the `contracts` in the
`MasterProto` and populate the table with the following:

```rust
tables
    .update_row("NftDeployment", contract.address)
    .set("name", contract.name)
    .set("symbol", contract.symbol)
    .set("blocknumber", contract.blocknumber)
    .set("timestamp", contract.timestamp_seconds);
```
