use crate::utils::backfill::backfill_blocks;
use crate::utils::server_handlers::{
    handle_get_block_by_hash, handle_get_block_by_id, handle_weave_gm,
};
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use tokio::task;

mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_headers(Any)
    .allow_origin(Any);

    let router = Router::new()
        .layer(cors)
        .route("/", get(handle_weave_gm))
        .route("/block/id/:id", get(handle_get_block_by_id))
        .route("/block/hash/:hash", get(handle_get_block_by_hash));

    task::spawn(async move {
        let _ = backfill_blocks(2500).await;
    });

    Ok(router.into())
}
