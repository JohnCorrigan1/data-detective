mod abi;
mod helpers;
mod pb;
use abi::erc721::events::Transfer as Erc721TransferEvent;
use helpers::erc721helpers::*;
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
fn erc721_out(blk: Block) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    blk.transaction_traces
        .iter()
        .filter(|tx| tx.status == 1)
        .for_each(|tx| {
            tx.calls
                .iter()
                .filter(|call| !call.state_reverted)
                .for_each(|call| {
                    let all_calls = tx.calls.as_ref();
                    call.logs.iter().for_each(|log| {
                        if let Some(transfer) = Erc721TransferEvent::match_and_decode(log) {
                            if transfer.from == NULL_ADDRESS {
                                tables
                                    .update_row(
                                        "NftToken",
                                        format!(
                                            "{}-{}",
                                            Hex::encode(&log.address),
                                            transfer.token_id
                                        ),
                                    )
                                    .set("tokenID", &transfer.token_id)
                                    .set("address", Hex::encode(&log.address))
                                    .set("blocknumber", blk.number)
                                    .set("timestamp", blk.timestamp_seconds())
                                    .set("collection", Hex::encode(&log.address));
                            }
                        }
                    });
                    if call.call_type == eth::CallType::Create as i32 {
                        if let Some(nft_contract) = ERC721Creation::from_call(all_calls, call) {
                            tables
                                .update_row("NftDeployment", Hex::encode(&call.address))
                                .set("code", nft_contract.code)
                                .set("blocknumber", blk.number)
                                .set("timestamp", blk.timestamp_seconds());

                            for change in nft_contract.storage_changes {
                                tables
                                    .update_row(
                                        "StorageChange",
                                        format!(
                                            "{}:{}",
                                            Hex::encode(&call.address),
                                            Hex::encode(&change.key)
                                        ),
                                    )
                                    .set("key", &change.key)
                                    .set("new_value", &change.new_value)
                                    .set("deployment", Hex::encode(&call.address));
                            }
                        }
                    }
                });
        });

    Ok(tables.to_entity_changes())
}
