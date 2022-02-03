use axum::{response::{ IntoResponse, Headers}, routing::{get, any}, Router, extract::{Path, RawBody}, http::{StatusCode, header::{self, HeaderName}, HeaderValue, Request, Response}, body::{Body, Bytes}};
use tracing::Level;
use std::{net::SocketAddr, string::FromUtf8Error};

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
   let collector = tracing_subscriber::fmt()
       .with_writer(non_blocking)
       .with_max_level(Level::INFO)
       .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    // build our application with a route
    let app = Router::new().route("/*any", any(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn decode_body(body: &Bytes) -> Result<String, FromUtf8Error>{
    String::from_utf8(body.to_vec())
}

async fn handler(Path(any): Path<String>, body: Bytes) ->  Response<Body> {
    let b = decode_body(&body).unwrap();
    tracing::error!("path: {}, b: {}", any, b);

    let body = b;
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, HeaderValue::from_static( mime::APPLICATION_JSON.as_ref()))
        .body(Body::from(body))
        .unwrap()
}