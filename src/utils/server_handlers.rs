use crate::utils::planetscale::{ps_get_block_by_hash, ps_get_block_by_id};
use axum::{extract::Path, response::Json};
use serde_json::{json, Value};

pub async fn handle_weave_gm() -> &'static str {
    "WeaveGM!"
}

pub async fn handle_get_block_by_id(Path(id): Path<u64>) -> Json<Value> {
    let tx_object = ps_get_block_by_id(id)
        .await
        .unwrap_or(json!({"error": "error fetching block from cloud"}));
    Json(tx_object)
}

pub async fn handle_get_block_by_hash(Path(hash): Path<String>) -> Json<Value> {
    let tx_object = ps_get_block_by_hash(hash.as_str())
        .await
        .unwrap_or(json!({"error": "error fetching block from cloud"}));
    Json(tx_object)
}
