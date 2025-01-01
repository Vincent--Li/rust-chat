use axum::response::IntoResponse;
use axum_macros::debug_handler;
use tracing::instrument;

#[debug_handler]
#[instrument]
pub(crate) async fn list_chat_handler() -> impl IntoResponse {
    "chat"
}

#[debug_handler]
#[instrument]
pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "chat"
}

#[debug_handler]
#[instrument]
pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "chat"
}

#[debug_handler]
#[instrument]
pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "chat"
}
