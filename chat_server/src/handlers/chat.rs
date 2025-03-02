use axum::{response::IntoResponse, Extension};
use axum_macros::debug_handler;
use tracing::{info, instrument};

use crate::User;

#[debug_handler]
#[instrument]
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    info!("user: {:?}", user);
    "list_chat_handler"
}

#[debug_handler]
#[instrument]
pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create_chat_handler"
}

#[debug_handler]
#[instrument]
pub(crate) async fn update_chat_handler(
) -> impl IntoResponse {
    "update_chat_handler"
}

#[debug_handler]
#[instrument]
pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete_chat_handler"
}
