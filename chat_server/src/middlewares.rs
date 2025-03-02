mod request_id;

use request_id::set_request_id;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use axum::{middleware::from_fn, Router};
use tracing::Level;


pub fn set_layers(app: Router) -> Router {
  app.layer(
      ServiceBuilder::new()
          // tracing 中间件用于打印日志等信息
          .layer(
              TraceLayer::new_for_http()
                  .make_span_with(DefaultMakeSpan::new().include_headers(true))
                  .on_request(DefaultOnRequest::new().level(Level::INFO))
                  .on_response(
                      DefaultOnResponse::new()
                          .level(Level::INFO)
                          .latency_unit(LatencyUnit::Millis),
                  ),
          )
          // compression中间件
          .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
          .layer(from_fn(set_request_id)),
  )
}