use axum::{extract::State, routing::get, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use vivs::Client;

struct AppState {
    cache: Client,
}

type SharedState = Arc<RwLock<AppState>>;

async fn get_handler(State(state): State<SharedState>) -> String {
    let mut app_state = state.write().await;

    let cache = &mut app_state.cache;
    let value = cache.get("name".to_owned()).await;

    if let Some(cached_value) = value {
        return cached_value;
    }

    "No value".to_owned()
}

#[tokio::main]
async fn main() {
    let vivs_client = Client::new().await;
    let vivs_client = vivs_client.unwrap();

    let app_state = Arc::new(RwLock::new(AppState { cache: vivs_client }));

    let app = Router::new()
        .route("/", get(get_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
