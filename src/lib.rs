mod config;

use std::{ops::Deref, sync::Arc};

use axum::{routing::{get, post}, Router};
use axum_macros::debug_handler;
pub use config::AppConfig;
use tracing::instrument;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
  inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
  #[allow(unused)]
  pub(crate) config: AppConfig,
}

pub fn get_router(config: AppConfig) -> Router {
  let state = AppState::new(config);

  let api = Router::new()
    .route("/signin", post(signin_handler))
    .route("/signup", post(signup_handler))
    .route("/chat", get(list_chat_handler));

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
    Self {
      inner: Arc::new(AppStateInner { config }),
    }
  }
}


#[debug_handler]
#[instrument]
async fn index_handler() -> String {
  format!("Hello, world! {:?}", "test")
}

#[debug_handler]
#[instrument]
async fn signin_handler() -> String {
  format!("Hello, world! {:?}", "test")
}

#[debug_handler]
#[instrument]
async fn signup_handler() -> String {
  format!("Hello, world! {:?}", "test")
}

#[debug_handler]
#[instrument]
async fn list_chat_handler() -> String {
  format!("Hello, world! {:?}", "test")
}
