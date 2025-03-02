use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{models::{CreateUser, SigninUser}, AppError, AppState, ErrorOutput, User};

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

    Ok((StatusCode::CREATED, Json(AuthOutput{token})))
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
        None => Ok((StatusCode::FORBIDDEN, Json(ErrorOutput::new("invalid email or password"))).into_response()),
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

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        assert!(!config.server.db_url.is_empty());
        let name = "vincent";
        let email = "vincent@gmail.com";
        let password = "123456";
        let (_tdb, state) = AppState::new_for_test(config).await?;
        let user = CreateUser::new(name, email, password);
        User::create(&user, &state.pool).await?;
        
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input)).await?.into_response();
        assert_eq!(ret.status(), StatusCode::OK);

        let body = ret.into_body().collect().await?.to_bytes();
    
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");

        Ok(())
    }

    #[tokio::test]
    async fn signup_duplicate_user_should_409() -> Result<()> {

        let config = AppConfig::load()?;
        assert!(!config.server.db_url.is_empty());
        let name = "vincent";
        let email = "vincent@gmail.com";
        let password = "123456";
        let (_tdb, state) = AppState::new_for_test(config).await?;
        let input = CreateUser::new(name, email, password);
        let _ = signup_handler(State(state.clone()), Json(input)).await?.into_response();
        let input = CreateUser::new(name, email, password);
        let ret = signup_handler(State(state.clone()), Json(input))
            .await
            .into_response();
        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let ret: ErrorOutput = serde_json::from_slice::<ErrorOutput>(&ret.into_body().collect().await?.to_bytes())?;
        assert_eq!(ret.error, "email already exists: vincent@gmail.com");
        Ok(())
    }

    #[tokio::test]
    async fn signin_invalid_user_should_403() -> Result<()> {
        let config = AppConfig::load()?;
        assert!(!config.server.db_url.is_empty());
        let email = "vincent@gmail.com";
        let password = "123456";
        let (_tdb, state) = AppState::new_for_test(config).await?;
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input)).await?.into_response();
        assert_eq!(ret.status(), StatusCode::FORBIDDEN);
        let ret: ErrorOutput = serde_json::from_slice::<ErrorOutput>(&ret.into_body().collect().await?.to_bytes())?;
        assert_eq!(ret.error, "invalid email or password");

        Ok(())
    }
}