mod rtb_model;
mod resource_selector;
mod app_conf;

use crate::{rtb_model::Video, app_conf::AppConf};
use axum::{
    body::{Body, Bytes},
    extract::{Path, Extension},
    http::{header, HeaderValue, Response, StatusCode, Uri},
    routing::any,
    AddExtensionLayer, Router,
};
use std::{sync::Arc, fs::File, io::BufReader, error::Error};
use std::{net::SocketAddr, string::FromUtf8Error};
use tracing::Level;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_appender = tracing_appender::rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    let path = "./config.yml";
    let file = File::open(path)?;
    let reader =BufReader::new(file);
    let raw_app_conf: app_conf::RawAppConf = serde_yaml::from_reader(reader)?;
    let app_conf = AppConf::from(&raw_app_conf);
    // build our application with a route
    let app = Router::new()
        .route("/*any", any(handler))
        .layer(AddExtensionLayer::new(app_conf.clone()));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

fn decode_body(body: &Bytes) -> Result<String, FromUtf8Error> {
    String::from_utf8(body.to_vec())
}


fn no_content() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap()
}


async fn handler(uri: Uri, Path(any): Path<String>, body: Bytes, Extension(app_conf): Extension<Arc<app_conf::AppConf>>) -> Response<Body> {
    let b = decode_body(&body).unwrap();
    let _v = Video {};
    tracing::error!("path: {}, b: {}", any, b);

    let _a = match app_conf.resources.iter().find(|x| x.uri == any) {
        Some(resource) => resource,
        None => {
            tracing::warn!("not found path: {}", any);
            return no_content();
        }
    };

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
