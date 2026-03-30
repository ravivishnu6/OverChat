use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

struct AppState {
    user_name: Mutex<String>,
}

#[derive(Deserialize)]
struct NameInput {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        user_name: Mutex::new("Guest".to_string()),
    });
    let app = Router::new()
        .route("/name", get(get_name))
        .route("/name", post(update_name))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🚀 Server with Memory running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_name(State(state): State<Arc<AppState>>) -> Json<GreetResponse> {
    let name = state.user_name.lock().unwrap();
    Json(GreetResponse {
        message: format!("The current user is {}", name),
    })
}

async fn update_name(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NameInput>,
) -> Json<GreetResponse> {
    let mut name = state.user_name.lock().unwrap();
    *name = payload.name.clone();

    Json(GreetResponse {
        message: format!("Name updated to {}", name),
    })
}
