use borsh::to_vec;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use planetscale_driver::Database;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub base_fee_per_gas: Option<String>,
    pub blob_gas_used: Option<String>,
    pub difficulty: Option<String>,
    pub excess_blob_gas: Option<String>,
    pub extra_data: Option<String>,
    pub gas_limit: Option<String>,
    pub gas_used: Option<String>,
    pub hash: Option<String>,
    pub logs_bloom: Option<String>,
    pub miner: Option<String>,
    pub mix_hash: Option<String>,
    pub nonce: Option<String>,
    pub number: Option<String>,
    pub parent_beacon_block_root: Option<String>,
    pub parent_hash: Option<String>,
    pub receipts_root: Option<String>,
    pub sha3_uncles: Option<String>,
    pub size: Option<String>,
    pub state_root: Option<String>,
    pub timestamp: Option<String>,
    pub total_difficulty: Option<String>,
    pub transactions: Option<Vec<Transaction>>,
    pub uncles: Option<Vec<String>>,
    pub withdrawals: Option<Vec<String>>,
    pub withdrawals_root: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, BorshSerialize, BorshDeserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash: Option<String>,
    pub block_number: Option<String>,
    pub chain_id: Option<String>,
    pub from: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub hash: Option<String>,
    pub input: Option<String>,
    pub nonce: Option<String>,
    pub r: Option<String>,
    pub s: Option<String>,
    pub to: Option<String>,
    pub transaction_index: Option<String>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    pub v: Option<String>,
    pub value: Option<String>,
}

impl Block {
    pub fn load_block_from_value(value: Value) -> Result<Block, serde_json::Error> {
        let block: Block = serde_json::from_value(value)?;
        Ok(block)
    }
    pub fn brotli_compress(input: &[u8]) -> Vec<u8> {
        let mut writer: brotli::CompressorWriter<Vec<u8>> =
            brotli::CompressorWriter::new(Vec::new(), 4096, 11, 22);
        writer.write_all(input).unwrap();
        writer.into_inner()
    }
    pub fn borsh_ser(input: &Block) -> Vec<u8> {
        to_vec(input).unwrap()
    }
}

#[derive(Database, Debug, Serialize, Deserialize)]
pub struct PlanetScaleBlock {
    pub block_hash: String,
    pub block_number: u64,
    pub arweave_hash: String,
}
