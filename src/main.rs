use std::{env, sync::Arc};

use axum::{Extension, Router, extract::DefaultBodyLimit, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::level_filters::LevelFilter;

use crate::upload_manager::UploadManager;

mod util;
mod auth;
mod bunny;
mod routes;
mod structs;
mod actions;
mod upload_manager;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
  dotenvy::dotenv()?;
  tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

  let app = Router::new()
    .route("/", get(routes::root::get))
    .route("/{bucket}/", get(routes::bucket::get))

    .fallback(routes::file::all)

    .layer(DefaultBodyLimit::max(100_000_000)) // 100MB
    .layer(TraceLayer::new_for_http())
    .layer(Extension(Arc::new(UploadManager::new())));

  let listener = TcpListener::bind(format!("0.0.0.0:{}", env::var("PORT").unwrap_or("8080".to_owned()))).await.unwrap();
  axum::serve(listener, app).await.unwrap();

  Ok(())
}