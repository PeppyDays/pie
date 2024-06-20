use axum::Router;

use crate::api::slack::event::get_event_router;

pub async fn get_router() -> Router {
    Router::new().nest("/slack", get_event_router().await)
}
