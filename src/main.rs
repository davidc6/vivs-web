use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    routing::{delete, get},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use vivs::Client;

struct AppState {
    cache: Client,
}

type SharedState = Arc<RwLock<AppState>>;

async fn get_handler(State(state): State<SharedState>) -> Json<Value> {
    let mut app_state = state.write().await;

    let cache = &mut app_state.cache;
    let value = cache.get("name".to_owned()).await;

    if let Some(cached_value) = value {
        return Json(json!({ "name": cached_value }));
    }

    Json(json!({ "name": "" }))
}

async fn delete_handler(State(state): State<SharedState>) -> Response {
    let mut app_state = state.write().await;

    let cache = &mut app_state.cache;
    let value = cache.delete("name".to_owned()).await;

    Response::default()
}

#[tokio::main]
async fn main() {
    let vivs_client = Client::new().await;
    let vivs_client = vivs_client.unwrap();

    let app_state = Arc::new(RwLock::new(AppState { cache: vivs_client }));

    let app = Router::new()
        .route("/", get(get_handler))
        .route("/", delete(delete_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
