use anyhow::Context;
use tokio::net::TcpListener;

use crate::api::router::get_router;

pub async fn run() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .context("Failed to bind listener")?;
    let router = get_router().await;
    axum::serve(listener, router)
        .await
        .context("Failed to serve application")?;

    Ok(())
}
