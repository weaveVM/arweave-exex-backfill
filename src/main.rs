use crate::utils::backfill::backfill_blocks;
use crate::utils::server_handlers::{handle_get_block_by_id, handle_weave_gm};
use axum::{routing::get, Router};
use tokio::task;

mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handle_weave_gm))
        .route("/block/id/:id", get(handle_get_block_by_id));

    task::spawn(async move {
        let _ = backfill_blocks(1000).await;
    });

    Ok(router.into())
}
