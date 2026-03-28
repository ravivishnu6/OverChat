use axum::{Json, Router, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct NameInput {
    name: String,
}

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[tokio::main]
async fn main() {
    let app = Router::new().route("/greet", post(greet_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("API is running on Localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn greet_handler(
    Json(payload): Json<NameInput>,
) -> Result<(StatusCode, Json<GreetResponse>), (StatusCode, Json<ErrorResponse>)> {
    if payload.name.len() < 3 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: String::from("Name must be atleast 3 characters long"),
            }),
        ));
    }

    let greeting = format!("Hello {}", payload.name);

    Ok((StatusCode::OK, Json(GreetResponse { message: greeting })))
}
