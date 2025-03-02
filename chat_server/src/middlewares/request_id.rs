use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;
use uuid::Uuid;

const X_REQUEST_ID: &str = "x-request-id";

pub async fn set_request_id(
    mut req: Request,
    next: Next,
) -> Response {
    // if x-request-id exists, do nothing, otherwise generate a new one with UUID
    let id = match req.headers().get(X_REQUEST_ID) {
        Some(v) => v.as_bytes().to_vec(),
        None => {
            let request_id = Uuid::now_v7().to_string();
            match request_id.parse() {
                Ok(v) => {
                  req.headers_mut().insert(
                    X_REQUEST_ID,
                    v
                  );
                }
                Err(e) => {
                  warn!("failed to parse request id: {}", e)
                }
            };
            request_id.as_bytes().to_vec()
        }
    };

    let mut res = next.run(req).await;
    match HeaderValue::from_bytes(&id) {
        Ok(v) => {
          res.headers_mut().insert(
            X_REQUEST_ID,
            v
          );
        }
        Err(e) => {
          warn!("failed to parse request id: {}", e)
        }
    };
    res  
}