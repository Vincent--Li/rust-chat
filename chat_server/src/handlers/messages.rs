use axum::response::IntoResponse;
use axum_macros::debug_handler;
use tracing::instrument;

#[debug_handler]
#[instrument]
pub(crate) async fn send_msg_handler() -> impl IntoResponse {
    "send_msg_handler"
}

#[debug_handler]
#[instrument]
pub(crate) async fn list_msg_handler() -> impl IntoResponse {
    "list_msg_handler"
}
