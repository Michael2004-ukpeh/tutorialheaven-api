mod db;
use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber::{*, EnvFilter};
use tracing::{info};
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).with_target(true).with_level(true).init();
  let pool = db::init_db().await?;
  let listener = tokio::net::TcpListener::bind("0.0.0:5000").await.unwrap();
  // let state = 
  let cors = CorsLayer::new().all_origins(Any).allow_methods(Any).allow_headers(Any);
  let app = Router::new()
  .route("/",get(|| async{"Hello World!"}))
  .layer(cors)
  .with_state(pool.clone());
    info!("Server running on 0.0.0:5000 🟢 ");
  axum::serve(listener,app).await.unwrap();

  Ok(())
}
