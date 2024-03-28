mod abi;
mod helpers;
mod pb;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use helpers::erc721helpers::*;
use pb::deployments::{Erc721Deployment, Erc721Mint, MasterProto};
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSet, StoreSetProto};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use substreams_ethereum::Event;
use substreams_ethereum::NULL_ADDRESS;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

#[substreams::handlers::map]
fn map_blocks(blk: Block) -> Result<MasterProto, substreams::errors::Error> {
    let mut deployments = Vec::new();
    let mut mints = Vec::new();
    blk.transaction_traces
        .iter()
        .filter(|tx| tx.status == 1)
        .for_each(|tx| {
            tx.calls
                .iter()
                .filter(|call| {
                    !call.state_reverted && call.call_type == eth::CallType::Create as i32
                })
                .for_each(|call| {
                    let all_calls = tx.calls.as_ref();
                    call.logs.iter().for_each(|log| {
                        if let Some(transfer) = Erc721TransferEvent::match_and_decode(log) {
                            if transfer.from == NULL_ADDRESS {
                                mints.push(Erc721Mint {
                                    token_id: transfer.token_id.to_string(),
                                    address: Hex::encode(&log.address),
                                    blocknumber: blk.number.to_string(),
                                    timestamp_seconds: blk.timestamp_seconds().to_string(),
                                });
                            }
                        }
                    });

                    if let Some(token) = ERC721Creation::from_call(all_calls, call) {
                        if let Some(deployment) = process_erc721_contract(token) {
                            deployments.push(deployment);
                        }
                    }
                });
        });

    Ok(MasterProto {
        mints,
        contracts: deployments,
    })
}

#[substreams::handlers::store]
pub fn store_contract_data(contracts: MasterProto, store: StoreSetProto<Erc721Deployment>) {
    for contract in contracts.contracts {
        store.set(0, &contract.address, &contract)
    }
}

#[substreams::handlers::map]
pub fn graph_out(
    master: MasterProto,
    store: StoreGetProto<Erc721Deployment>,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    for mint in master.mints.iter() {
        if let Some(contract) = store.get_at(0, &mint.address) {
            let token_uri = match get_token_uri(&contract, &mint.token_id) {
                Ok(token_uri) => token_uri,

                Err(_e) => String::new(),
            };
            tables.update_row("NftToken", format!("{}-{}", mint.address, mint.token_id))
            .set("tokenID", mint.token_id.clone())
            .set("tokenURI", token_uri)
            .set("address", mint.address.clone())
            .set("blocknumber", BigInt::from_str(&mint.blocknumber).unwrap_or(BigInt::from(0)))
            .set("timestamp_seconds", BigInt::from_str(&mint.timestamp_seconds).unwrap_or(BigInt::from(0)))
            .set("collection", mint.address.clone());
        }
    }
    for contract in master.contracts  {
        tables.update_row("NftDeployments", contract.address)
        .set("name", contract.name)
        .set("symbol", contract.symbol)
        .set("blocknumber", BigInt::from_str(&contract.blocknumber).unwrap_or(BigInt::from(0)))
        .set("timestamp_seconds", BigInt::from_str(&contract.timestamp_seconds).unwrap_or(BigInt::from(0)));
    }
    Ok(tables.to_entity_changes())
}
