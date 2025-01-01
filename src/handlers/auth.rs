use axum::response::IntoResponse;
use axum_macros::debug_handler;
use tracing::instrument;

#[debug_handler]
#[instrument]
pub(crate) async fn signin_handler() -> impl IntoResponse {
    "signin"
}

#[debug_handler]
#[instrument]
pub(crate) async fn signup_handler() -> impl IntoResponse {
    "signup"
}
