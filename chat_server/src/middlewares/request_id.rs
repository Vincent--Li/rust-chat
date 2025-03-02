use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;
use uuid::Uuid;

use super::X_REQUEST_ID;

pub async fn set_request_id(
    mut req: Request,
    next: Next,
) -> Response {
    // if x-request-id exists, do nothing, otherwise generate a new one with UUID
    // if x-request-id exists, do nothing, otherwise generate a new one

    let id = match req.headers().get(X_REQUEST_ID) {
      Some(v) => Some(v.clone()),
      None => {
          let request_id = Uuid::now_v7().to_string();
          match HeaderValue::from_str(&request_id) {
              Ok(v) => {
                  req.headers_mut().insert(X_REQUEST_ID, v.clone());
                  Some(v)
              }
              Err(e) => {
                  warn!("parse generated request id failed: {}", e);
                  None
              }
          }
      }
  };

  let mut res = next.run(req).await;

  let Some(id) = id else {
      return res;
  };
  res.headers_mut().insert(X_REQUEST_ID, id);
  res
}