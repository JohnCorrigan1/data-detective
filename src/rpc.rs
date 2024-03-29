use crate::abi;
use crate::helpers;
use crate::pb::deployments::Erc721Deployment;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use abi::erc721::functions::{Name, Symbol, TokenUri, TotalSupply};
pub use helpers::erc721helpers::contains_erc721_fns;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;
use substreams_ethereum::rpc::RpcBatch;

use substreams_ethereum::Event;
use substreams_ethereum::NULL_ADDRESS;

#[substreams::handlers::map]
#[substreams::handlers::map]
pub fn map_erc721_rpc(blk: Block) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    blk.transaction_traces
        .iter()
        .filter(|tx| tx.status == 1)
        .for_each(|tx| {
            tx.calls
                .iter()
                .filter(|call| !call.state_reverted)
                .for_each(|call| {
                    if call.call_type == eth::CallType::Create as i32 {
                        if let Some(last_code_change) = call.code_changes.iter().last() {
                            let code = &last_code_change.new_code;
                            // let address = &call.address.to_vec();
                            if contains_erc721_fns(&Hex::encode(&code)) {
                                let nft = get_erc721_deployment(call.address.clone());
                                //
                                tables
                                    .update_row("NftDeployment", nft.address)
                                    .set("name", nft.name)
                                    .set("symbol", nft.symbol)
                                    .set("blocknumber", blk.number)
                                    .set("timestamp", blk.timestamp_seconds());
                            }
                        }
                    }

                    call.logs.iter().for_each(|log| {
                        if let Some(erc721_transfer) = Erc721TransferEvent::match_and_decode(log) {
                            if erc721_transfer.from == NULL_ADDRESS {
                                let token_uri =
                                    nft_uri(log.address.clone(), erc721_transfer.token_id.clone());

                                let address = Hex::encode(&log.address);
                                let token_id = erc721_transfer.token_id.to_string();

                                tables
                                    .update_row("NftToken", &format!("{}:{}", &address, &token_id))
                                    .set("tokenId", &token_id)
                                    .set("tokenURI", token_uri)
                                    .set("address", &address)
                                    .set("blocknumber", blk.number)
                                    .set("timestamp", blk.timestamp_seconds())
                                    .set("collection", &address);
                            }
                        }
                    })
                })
        });
    Ok(tables.to_entity_changes())
}

pub fn nft_uri(address: Vec<u8>, token_id: BigInt) -> String {
    let batch = RpcBatch::new();
    let response = batch
        .add(TokenUri { token_id: token_id }, address)
        .execute();

    if let Some(nft) = response.ok() {
        let nft = nft.responses;
        return RpcBatch::decode::<_, TokenUri>(&nft[0]).unwrap_or(String::new());
    }
    String::new()
}

pub fn get_erc721_deployment(address: Vec<u8>) -> Erc721Deployment {
    let batch = RpcBatch::new();
    let response = batch
        .add(Name {}, address.clone())
        .add(Symbol {}, address.clone())
        .add(TotalSupply {}, address.clone())
        .execute();

    if let Some(nft) = response.ok() {
        let nft = nft.responses;
        return Erc721Deployment {
            address: Hex::encode(&address),
            name: RpcBatch::decode::<_, Name>(&nft[0]).unwrap_or(String::new()),
            symbol: RpcBatch::decode::<_, Symbol>(&nft[1]).unwrap_or(String::new()),
            blocknumber: 0,
            code: Vec::new(),
            storage_changes: Vec::new(),
            timestamp_seconds: 0,
        };
    }
    Erc721Deployment {
        address: Hex::encode(&address),
        name: String::new(),
        symbol: String::new(),
        blocknumber: 0,
        code: Vec::new(),
        storage_changes: Vec::new(),
        timestamp_seconds: 0,
    }
}
