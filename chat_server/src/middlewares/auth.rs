use axum::{extract::{FromRequestParts as _, Query, Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};
use axum_core::extract::FromRequestParts;
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use serde::Deserialize;
use tracing::warn;

use crate::AppState;


#[derive(Debug, Deserialize)]
struct Params {
    token: String,
}

pub(crate) async fn verify_token(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let (mut parts, body) = req.into_parts();
    let token =
      match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
          Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
          Err(e) => {
              if e.is_missing() {
                  match Query::<Params>::from_request_parts(&mut parts, &state).await {
                      Ok(params) => params.token.clone(),
                      Err(e) => {
                          let msg = format!("parse query params failed: {}", e);
                          warn!(msg);
                          return (StatusCode::UNAUTHORIZED, msg).into_response();
                      }
                  }
              } else {
                  let msg = format!("parse Authorization header failed: {}", e);
                  warn!(msg);
                  return (StatusCode::UNAUTHORIZED, msg).into_response();
              }
          }
      };

      let req = match state.dk.verify(&token) {
      Ok(user) => {
          let mut req = Request::from_parts(parts, body);
          req.extensions_mut().insert(user);
          req
      }
      Err(e) => {
          let msg = format!("verify token failed: {:?}", e);
          warn!(msg);
          return (StatusCode::FORBIDDEN, msg).into_response();
      }
  };

  next.run(req).await
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{body::Body, middleware::from_fn_with_state, routing::get, Router};
    use tower::ServiceExt;

    use crate::{AppConfig, User};

    use super::*;

    async fn handler(_req: Request) -> impl IntoResponse {
      (StatusCode::OK, "ok")
    }

    #[tokio::test]
    async fn verify_token_middleware_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        assert!(!config.server.db_url.is_empty());
        let (_tdb, state) = AppState::new_for_test(config).await?;

        let user = User::new(1i64, "vincent", "vincent@test.com");
        let token = state.ek.sign_token(user)?;

        let app = Router::new()
            .route("/", get(handler))
            .layer(from_fn_with_state(state.clone(),verify_token))
            .with_state(state);

        // good token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK); 

        // no token
        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        // bad token
        let req = Request::builder()
            .uri("/")
            .header("Authorization", "Bearer badtoken")
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        // query params
        let req = Request::builder()
            .uri("/?token=badtoken")
            .body(Body::empty())?;
        let res = app.clone().oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::FORBIDDEN);

        // query with good token
        let req = Request::builder()
            .uri("/?token=".to_owned() + &token)
            .body(Body::empty())?;
        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);


        Ok(())
    }
}