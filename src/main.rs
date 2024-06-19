use axum::{extract::Json, routing::post, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = Router::new().route("/events", post(handle_event_message));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handle_event_message(
    Json(message): Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    println!("Received events: {:?}", message);

    match message.get("challenge") {
        Some(challenge) => axum::Json(serde_json::json!({"challenge": challenge})),
        _ => axum::Json(serde_json::json!({})),
    }
}
