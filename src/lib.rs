mod abi;
mod helpers;
mod pb;
use abi::erc20::functions::{Decimals, Name as TokenName, Symbol as TokenSymbol, TotalSupply};
use abi::erc721::functions::{Name, Symbol};
use helpers::erc20helpers::*;
use helpers::erc721helpers::*;
use std::str::FromStr;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use substreams_ethereum::rpc::RpcBatch;

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
                        if let Some(deployment) = process_erc20_contract(token) {
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
                                .set("blocknumber", blk.number)
                                .set("timestamp", blk.timestamp_seconds());
                        }
                    } else if let Some(token) = ERC721Creation::from_call(all_calls, call) {
                        if let Some(deployment) = process_erc721_contract(token) {
                            tables
                                .update_row("NftDeployment", deployment.address)
                                .set("name", deployment.name)
                                .set("symbol", deployment.symbol)
                                .set("blocknumber", blk.number)
                                .set("timestamp", blk.timestamp_seconds());
                        }
                    }
                });
        });

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
fn map_deployments_rpc(blk: Block) -> Result<EntityChanges, substreams::errors::Error> {
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
                    if let Some(token_metadata) = ERC20Creation::from_call(call) {
                        let token = get_erc20_calls(token_metadata.address.clone());

                        tables
                            .update_row("TokenDeployment", Hex::encode(&token_metadata.address))
                            .set("name", token.name)
                            .set("symbol", token.symbol)
                            .set("decimals", token.decimals)
                            .set("totalSupply", token.total_supply)
                            .set("blocknumber", blk.number)
                            .set("timestamp", blk.timestamp().seconds);
                    } else if let Some(last_code_change) = call.code_changes.iter().last() {
                        let code = &last_code_change.new_code;
                        let address = &call.address.to_vec();
                        if contains_erc721_fns(&Hex::encode(&code)) {
                            let nft = get_erc721_calls(address.clone());
                            tables
                                .update_row("NftDeployment", Hex::encode(address))
                                .set("name", nft.name)
                                .set("symbol", nft.symbol)
                                .set("blocknumber", blk.number)
                                .set("timestamp", blk.timestamp().seconds);
                        }
                    }
                });
        });

    Ok(tables.to_entity_changes())
}

struct Token {
    name: String,
    symbol: String,
    decimals: BigInt,
    total_supply: BigInt,
}

fn get_erc20_calls(address: Vec<u8>) -> Token {
    let batch = RpcBatch::new();
    let response = batch
        .add(TokenName {}, address.clone())
        .add(TokenSymbol {}, address.clone())
        .add(Decimals {}, address.clone())
        .add(TotalSupply {}, address)
        .execute();

    if let Some(response) = response.ok() {
        let response = response.responses;
        return Token {
            name: RpcBatch::decode::<_, TokenName>(&response[0])
                .unwrap_or(String::from("Name not found"))
                .to_string(),
            symbol: RpcBatch::decode::<_, TokenSymbol>(&response[1])
                .unwrap_or(String::from("Symbol not found"))
                .to_string(),
            decimals: RpcBatch::decode::<_, Decimals>(&response[2]).unwrap_or(BigInt::zero()),
            total_supply: RpcBatch::decode::<_, TotalSupply>(&response[3])
                .unwrap_or(BigInt::zero()),
        };
    }
    Token {
        name: String::from("Name not found"),
        symbol: String::from("Symbol not found"),
        decimals: BigInt::zero(),
        total_supply: BigInt::zero(),
    }
}
struct NFT {
    name: String,
    symbol: String,
}

fn get_erc721_calls(address: Vec<u8>) -> NFT {
    let batch = RpcBatch::new();
    let response = batch
        .add(Name {}, address.clone())
        .add(Symbol {}, address.clone())
        .execute();

    if let Some(response) = response.ok() {
        let response = response.responses;
        return NFT {
            name: RpcBatch::decode::<_, Name>(&response[0])
                .unwrap_or(String::from("Name not found"))
                .to_string(),
            symbol: RpcBatch::decode::<_, Symbol>(&response[1])
                .unwrap_or(String::from("Symbol not found"))
                .to_string(),
        };
    }
    NFT {
        name: String::from("Name not found"),
        symbol: String::from("Symbol not found"),
    }
}
