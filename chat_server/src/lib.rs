mod config;
mod error;
mod handlers;
mod models;
mod utils;

use anyhow::Context;
use handlers::*;
use serde::ser;
use sqlx::{pool, PgPool};
use std::{
    fmt::{self, Formatter},
    ops::Deref,
    sync::Arc,
};
use utils::{DecodingKey, EncodingKey};

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

#[allow(unused)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
    pub(crate) pool: PgPool,
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;

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

    Ok(Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state))
}

// 当使用 state.config 实际上自动引用了 state.inner.config 中的值
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
        pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let ek = EncodingKey::load_pem(&config.auth.pk).context("load ek failed")?;
        let dk = DecodingKey::load_pem(&config.auth.sk).context("load sk failed")?;
        let pool = PgPool::connect(&config.server.db_url)
            .await
            .context("connect to db failed")?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        })
    }

    
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}

#[cfg(test)]
impl AppState {
    pub async fn new_for_test(config: AppConfig) -> Result<(sqlx_db_tester::TestPg, Self), AppError> {
        use std::path::Path;
        use sqlx_db_tester::TestPg;
    
        let ek = EncodingKey::load_pem(&config.auth.sk).context("test load sk failed")?;

        let dk = DecodingKey::load_pem(&config.auth.pk).context("test load pk failed")?;
        let post = config.server.db_url.rfind('/').expect("invalid db_url");

        let server_url = String::from(&config.server.db_url[..post]) + "/test";
        let tdb = TestPg::new(
            server_url,
             Path::new("../migrations")
            );
        let pool = tdb.get_pool()
            .await;
        let state = Self {
            inner: Arc::new(AppStateInner {
                config,
                ek,
                dk,
                pool,
            }),
        };
        Ok((tdb, state))
    }
}

