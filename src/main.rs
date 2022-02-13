mod app_conf;
mod arg_option;
mod resource_selector;
mod rtb_model;

use crate::{
    app_conf::{read_app_conf},
    rtb_model::Request,
};
use axum::{
    body::{Body, Bytes},
    extract::Extension,
    http::{header, HeaderMap, HeaderValue, Method, Response, StatusCode, Uri},
    routing::any,
    AddExtensionLayer, Router,
};
use clap::StructOpt;
use std::{error::Error, sync::Arc};
use std::{net::SocketAddr, string::FromUtf8Error};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_option::Args::parse();

    let file_appender = tracing_appender::rolling::never("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    tracing::info!("args: {:?}", args);

    let app_conf = read_app_conf(&args.conf_path)?;

    for r in app_conf.resources.iter() {
        println!("path: {}, imp_condition: {:?}", r.uri, r.imp_condition);
    }

    // build our application with a route
    let app = Router::new()
        .fallback(any(handler))
        .layer(AddExtensionLayer::new(Arc::new(app_conf.clone())));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
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

fn build_response_with_body(body: String) -> Response<Body> {
    return Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )
        .body(Body::from(body))
        .unwrap();
}

async fn handler(
    uri: Uri,
    // Path(any): Path<String>,
    body_bytes: Bytes,
    method: Method,
    headers: HeaderMap,
    Extension(app_conf): Extension<Arc<app_conf::AppConf>>,
) -> Response<Body> {
    let body = decode_body(&body_bytes).unwrap();
    tracing::info!(
        "uri: {}, method: {}, body: {}, header: {:?}",
        uri,
        method,
        body,
        headers
    );
    let target_resource = match app_conf.resources.iter().find(|x| x.uri == uri.path()) {
        Some(resource) => resource,
        None => {
            tracing::warn!("not found path. uri: {}", uri);
            return build_response(StatusCode::NO_CONTENT);
        }
    };
    if method != Method::POST {
        tracing::info!(
            "ignore imp_condition since method is {}. uri: {}, request: {}, response: {}",
            uri,
            method,
            body,
            target_resource.content
        );
        return build_response_with_body(target_resource.content.clone());
    }

    let request: Request = match serde_json::from_str(&body) {
        Ok(req) => req,
        Err(err) => {
            tracing::error!("parse failed. body: {}, err: {}", body, err);
            return build_response(StatusCode::BAD_REQUEST);
        }
    };

    let response_content_body =
        match resource_selector::select_resource_with_replacing_macro(target_resource, &request) {
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
        response_content_body
    );
    build_response_with_body(response_content_body)
}
