mod config;
mod error;
mod handlers;
mod models;
mod utils;

use handlers::*;
use utils::{DecodingKey, EncodingKey};
use std::{fmt::{self, Formatter}, ops::Deref, sync::Arc};

pub use error::AppError;
pub use models::User;

use axum::{
    routing::{get, patch, post},
    Router,
};

pub use config::AppConfig;


#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_msg_handler),
        )
        .route("/chat/:id/messages", get(list_msg_handler));

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}

// 当使用 state.config 实际上自动引用了 state.inner.config 中的值
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let ek = EncodingKey::load_pem(&config.auth.pk).expect(" load pk error");
        let dk = DecodingKey::load_pem(&config.auth.sk).expect(" load sk error");
        Self {
            inner: Arc::new(AppStateInner { config, ek, dk }),
        }
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
