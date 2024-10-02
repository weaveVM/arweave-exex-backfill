use crate::utils::arweave_gql::detect_missing_blocks;
use crate::utils::arweave_upload::{ArweaveRequest, UploaderProvider};
use crate::utils::block::Block;
use crate::utils::constants::{RETH_CLIENT_VERSION, WVM_NETWORK_TAG};
use crate::utils::planetscale::{ps_init, ps_insert_block};
use crate::utils::wvm_client::{block_hex_to_decimal, retrieve_block_with_txs};
use anyhow::{Error, Ok};

pub async fn backfill_blocks(scan_count: u32) -> Result<(), Error> {
    let ar_uploader_provider = UploaderProvider::new(None);
    let missed_blocks = detect_missing_blocks(scan_count).await.unwrap();
    // planetscale connection
    let conn = ps_init().await;

    for block_number in missed_blocks.iter() {
        
        let wvm_block = retrieve_block_with_txs(*block_number).await;
        let block_number_hex: &str = wvm_block.number.as_ref().unwrap();
        let block_number = block_hex_to_decimal(block_number_hex.as_ref());
        let block_hash = wvm_block.hash.as_ref().unwrap().as_str();
        let borsh_block = Block::borsh_ser(&wvm_block);
        let borsh_brotli = Block::brotli_compress(&borsh_block);

        let arweave_id = ArweaveRequest::new()
            .set_tag("Content-Type", "application/octet-stream")
            .set_tag("WeaveVM:Encoding", "Borsh-Brotli")
            .set_tag("Block-Number", block_number.as_str())
            .set_tag("Block-Hash", block_hash)
            .set_tag("Client-Version", RETH_CLIENT_VERSION)
            .set_tag("Network", WVM_NETWORK_TAG)
            .set_tag("WeaveVM:Backfill", "true")
            .set_data(borsh_brotli)
            .send_with_provider(&ar_uploader_provider)
            .await
            .unwrap();

        println!("\n\nARWEAVE TXID: {}\n\n", arweave_id);
        let parsed_block_number = block_number.parse::<u64>()?;
        ps_insert_block(&conn, parsed_block_number, block_hash, &arweave_id)
            .await
            .unwrap();
    }

    Ok(())
}
