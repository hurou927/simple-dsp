mod app_conf;
mod resource_selector;
mod rtb_model;

use crate::{
    app_conf::AppConf,
    rtb_model::Request,
};
use axum::{
    body::{Body, Bytes},
    extract::{Extension, Path},
    http::{header, HeaderValue, Response, StatusCode, Uri},
    routing::any,
    AddExtensionLayer, Router,
};
use std::{error::Error, fs::File, io::BufReader, sync::Arc};
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
    let reader = BufReader::new(file);
    let raw_app_conf: app_conf::RawAppConf = serde_yaml::from_reader(reader)?;
    let app_conf = AppConf::from(&raw_app_conf);

    for r in app_conf.resources.iter() {
        println!("path: {}, imp_condition: {:?}", r.uri, r.imp_condition);
    }

    // build our application with a route
    let app = Router::new()
        .route("/*any", any(handler))
        .layer(AddExtensionLayer::new(Arc::new(app_conf.clone())));

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

fn build_response(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}

async fn handler(
    uri: Uri,
    // Path(any): Path<String>,
    body_bytes: Bytes,
    Extension(app_conf): Extension<Arc<app_conf::AppConf>>,
) -> Response<Body> {
    let body = decode_body(&body_bytes).unwrap();
    tracing::info!("uri: {}, body: {}", uri, body);

    let request: Request = match serde_json::from_str(&body) {
        Ok(req) => req,
        Err(err) => {
            tracing::error!("parse failed. body: {}, err: {}", body, err);
            return build_response(StatusCode::BAD_REQUEST);
        }
    };

    let resource = match app_conf.resources.iter().find(|x| x.uri == uri.path()) {
        Some(resource) => resource,
        None => {
            tracing::warn!("not found path. uri: {}", uri);
            return build_response(StatusCode::NO_CONTENT);
        }
    };

    let returned_resource =
        match resource_selector::select_resource_with_replacing_macro(resource, &request) {
            Some(resource) => resource,
            None => {
                tracing::warn!("not found resource");
                return build_response(StatusCode::NO_CONTENT);
            }
        };
    tracing::info!(
        "uri: {}, request: {}, response: {}",
        uri,
        body,
        returned_resource
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )
        .body(Body::from(returned_resource))
        .unwrap()
}
