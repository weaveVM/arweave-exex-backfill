use crate::utils::backfill::backfill_blocks;
use crate::utils::server_handlers::{
    handle_get_block_by_hash, handle_get_block_by_id, handle_weave_gm,
};
use axum::{routing::get, Router};
use http::Method;
use std::thread;
use std::time::Duration;
use tokio::task;
use tower_http::cors::{Any, CorsLayer};

mod utils;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // load secrets from Shuttle.toml into env var;
    secrets.into_iter().for_each(|(key, val)| {
        println!("{:?} {:?}", key, val);
        std::env::set_var(key, val);
    });

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
        loop {
            let _ = backfill_blocks(10_000).await;
            thread::sleep(Duration::from_secs(7200)); // 2h
        }
    });

    Ok(router.into())
}
