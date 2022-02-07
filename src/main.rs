mod rtb_model;
mod resource_selector;
mod app_conf;

use crate::rtb_model::Video;
use axum::{
    body::{Body, Bytes},
    extract::{Path, Extension},
    http::{header, HeaderValue, Response, StatusCode},
    routing::any,
    AddExtensionLayer, Router,
};
use std::sync::Arc;
use std::{net::SocketAddr, string::FromUtf8Error};
use tracing::Level;


struct AppConfig {
    // resource_conf: resource_selector::Conf,
    d: i32
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    let state = Arc::new(AppConfig { d: 1 });
    // build our application with a route
    let app = Router::new()
        .route("/*any", any(handler))
        .layer(AddExtensionLayer::new(state));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn decode_body(body: &Bytes) -> Result<String, FromUtf8Error> {
    String::from_utf8(body.to_vec())
}

async fn handler(Path(any): Path<String>, body: Bytes, Extension(state): Extension<Arc<AppConfig>>) -> Response<Body> {
    let b = decode_body(&body).unwrap();
    let _v = Video {};
    tracing::error!("path: {}, b: {}", any, b);

    let body = b;
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )
        .body(Body::from(body))
        .unwrap()
}
