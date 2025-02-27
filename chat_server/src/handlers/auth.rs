use axum::{extract::State, http::{HeaderMap, StatusCode}, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::instrument;

use crate::{models::{CreateUser, SigninUser}, AppError, AppState, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
}


#[debug_handler]
#[instrument]
pub(crate) async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&input, &state.pool).await?;
    let token = state.ek.sign_token(user)?;

    Ok((StatusCode::CREATED, Json(AuthOutput{ token})))
}

#[debug_handler]
#[instrument]
pub(crate) async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user= User::verify(&input, &state.pool).await?;
    match user {
        Some(user) => {
            let token = state.ek.sign_token(user)?;
            Ok((StatusCode::OK, Json(AuthOutput{ token})).into_response())
        }
        None => Ok((StatusCode::FORBIDDEN, "invalid email or password").into_response()),
    }
}

#[cfg(test)]
mod tests {
    use crate::AppConfig;

    use super::*;
    use anyhow::Result;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn signup_should_work() -> Result<()>{
        let config = AppConfig::load()?;
        assert!(!config.server.db_url.is_empty());

        let (_tdb, state) = AppState::new_for_test(config).await?;

        let input = CreateUser::new("Vincent", "vincent@gmail.com", "password");
        let ret = signup_handler(State(state), Json(input)).await?.into_response();
        assert_eq!(ret.status(), StatusCode::CREATED);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;

        assert_ne!(ret.token, "");

        Ok(())
    }

    // #[tokio::test]
    // async fn signin_should_work() -> Result<()> {

    // }
}