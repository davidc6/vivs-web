use axum::{routing::get, Extension, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use vivs::Client;

struct AppState {
    cache: Client,
}

type SharedState = Arc<RwLock<AppState>>;

async fn get_handler(Extension(state): Extension<SharedState>) -> String {
    let mut l = state.write().await;
    let cache = &mut l.cache;
    let value = cache.get("name".to_owned()).await;
    if let Some(cached_value) = value {
        cached_value
    } else {
        "No value".to_owned()
    }
}

#[tokio::main]
async fn main() {
    let vivs_client = Client::new().await;
    let vivs_client = vivs_client.unwrap();

    let app_state = Arc::new(RwLock::new(AppState { cache: vivs_client }));

    let app = Router::new()
        .route("/", get(get_handler))
        .layer(Extension(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
