mod auth;
mod chat;
mod messages;

pub(crate) use auth::*;
use axum_macros::debug_handler;
pub(crate) use chat::*;
pub(crate) use messages::*;
use tracing::instrument;

#[debug_handler]
#[instrument]
pub(crate) async fn index_handler() -> String {
    format!("Hello, world! {:?}", "test")
}
