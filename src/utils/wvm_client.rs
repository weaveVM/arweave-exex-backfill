use crate::utils::block::Block;
use crate::utils::constants::WVM_RPC_URL;
use anyhow::Error;
use reqwest::Client;
use serde_json::json;

pub async fn retrieve_block_with_txs(block_number: u32) -> Block {
    let block_number_hex = format!("0x{:x}", block_number);

    // JSON-RPC request payload
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [block_number_hex, true],  // true to get full transactions
        "id": 1
    });

    // Create an HTTP client
    let client = Client::new();
    let res = client
        .post(WVM_RPC_URL)
        .json(&request_body)
        .send()
        .await
        .unwrap();

    let result = res.json::<serde_json::Value>().await.unwrap();
    let wvm_block_fmt = Block::load_block_from_value(result["result"].clone()).unwrap();
    println!("{:#?}", wvm_block_fmt);

    wvm_block_fmt
}

pub async fn get_latest_block_number() -> Result<u64, Error> {
    // JSON-RPC request payload
    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });

    // Create an HTTP client
    let client = Client::new();
    let res = client.post(WVM_RPC_URL).json(&request_body).send().await?;

    let result = res.json::<serde_json::Value>().await?;

    // Extract the block number from the result
    let block_number_hex = result["result"]
        .as_str()
        .ok_or_else(|| Error::msg("Failed to get block number as string"))?;

    // Convert hex string to u64
    let block_number = u64::from_str_radix(&block_number_hex[2..], 16)?;

    println!("Latest block number: {}", block_number);

    Ok(block_number)
}

pub fn block_hex_to_decimal(hex_str: &str) -> String {
    let decimal_value =
        u64::from_str_radix(&hex_str[2..], 16).expect("Failed to convert hex to decimal");
    decimal_value.to_string()
}
