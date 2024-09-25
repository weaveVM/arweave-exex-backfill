use crate::utils::block::PlanetScaleBlock;
use crate::utils::env_var::get_env_var;
use anyhow::Error;
use planetscale_driver::{query, PSConnection};
use serde_json::Value;

pub async fn ps_init() -> PSConnection {
    let host = get_env_var("DATABASE_HOST").unwrap();
    let username = get_env_var("DATABASE_USERNAME").unwrap();
    let password = get_env_var("DATABASE_PASSWORD").unwrap();

    let conn: PSConnection = PSConnection::new(&host, &username, &password);

    conn
}

pub async fn ps_insert_block(
    conn: &PSConnection,
    block_number: u64,
    block_hash: &str,
    arweave_hash: &str,
) -> Result<(), Error> {
    let block_hash = block_hash.strip_prefix("0x").unwrap_or(block_hash);

    println!(
        "Attempting to insert block: block_hash = {}, block_number = {}, arweave_hash = {}",
        block_hash, block_number, arweave_hash
    );

    let insert_query = format!(
        "INSERT INTO ExExBackfill(BlockHash, BlockNumber, ArweaveHash) VALUES ('{}', {}, '{}')",
        block_hash, block_number, arweave_hash
    );

    match query(&insert_query).execute(conn).await {
        Ok(_) => {
            println!("Successfully inserted block {}", block_number);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error inserting block {}: {:?}", block_number, e);
            // Log the error and continue without halting
            Ok(())
        }
    }
}

pub async fn ps_get_block_by_id(block_number: u64) -> Result<Value, Error> {
    let conn = ps_init().await;
    let select_query = format!(
        "SELECT BlockHash, BlockNumber, ArweaveHash FROM ExExBackfill WHERE BlockNumber = {}",
        block_number
    );

    let block: PlanetScaleBlock = query(&select_query)
        .fetch_one(&conn)
        .await
        .unwrap_or(PlanetScaleBlock::empty());
    let res = serde_json::json!(block);
    Ok(res)
}

pub async fn ps_get_block_by_hash(block_hash: &str) -> Result<Value, Error> {
    let block_hash = block_hash
        .strip_prefix("0x")
        .unwrap_or(&block_hash)
        .to_string();
    let conn = ps_init().await;
    let select_query = format!(
        "SELECT BlockHash, BlockNumber, ArweaveHash FROM ExExBackfill WHERE BlockHash = '{}'",
        block_hash
    );

    let block: PlanetScaleBlock = query(&select_query)
        .fetch_one(&conn)
        .await
        .unwrap_or(PlanetScaleBlock::empty());
    let res = serde_json::json!(block);
    Ok(res)
}
