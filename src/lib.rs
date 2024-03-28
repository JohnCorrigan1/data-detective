mod abi;
mod helpers;
mod pb;
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

pub struct ContractCreation {
    pub address: String,
    pub bytecode: String,
    pub abi: String,
}

#[substreams::handlers::map]
fn map_deployments(blk: Block) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
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

                    if let Some(token) = ERC20Creation::from_call(call) {
                        if let Some(deployment) =
                            process_erc20_contract(token, blk.number, blk.timestamp().seconds)
                        {
                            tables
                                .update_row("TokenDeployment", deployment.address)
                                .set("name", deployment.name)
                                .set("symbol", deployment.symbol)
                                .set(
                                    "decimals",
                                    BigInt::from_str(&deployment.decimals)
                                        .unwrap_or(BigInt::zero()),
                                )
                                .set(
                                    "totalSupply",
                                    BigInt::from_str(&deployment.total_supply)
                                        .unwrap_or(BigInt::zero()),
                                )
                                .set(
                                    "blocknumber",
                                    BigInt::from_str(&deployment.blocknumber).unwrap(),
                                )
                                .set("timestamp", deployment.timestamp_seconds);
                        }
                    } else if let Some(token) = ERC721Creation::from_call(all_calls, call) {
                        if let Some(deployment) =
                            process_erc721_contract(token, blk.number, blk.timestamp().seconds)
                        {
                            tables
                                .update_row("NftDeployment", deployment.address)
                                .set("name", deployment.name)
                                .set("symbol", deployment.symbol)
                                .set(
                                    "blocknumber",
                                    BigInt::from_str(&deployment.blocknumber).unwrap(),
                                )
                                .set("timestamp", deployment.timestamp_seconds);
                        }
                    }
                });
        });

    Ok(tables.to_entity_changes())
}
