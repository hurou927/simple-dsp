use crate::{app_conf::AppConf, content_builder, rtb_model::Request};
use axum::{
    body::{Body, Bytes},
    extract::Extension,
    http::{header, HeaderMap, HeaderValue, Method, Response, StatusCode, Uri},
};
use flate2::read::GzDecoder;
use std::error::Error;
use std::io::Read;
use std::sync::Arc;

fn decode_body(body: &Bytes, headers: &HeaderMap) -> Result<String, Box<dyn Error>> {
    let content_encoding_opt = headers
        .get(header::CONTENT_ENCODING)
        .map(|hv| hv.to_str().ok()) // ignore to_str error
        .flatten()
        .map(|hv| hv.to_lowercase());

    let decoded = match content_encoding_opt.as_deref() {
        Some("identity") | None => String::from_utf8(body.to_vec())?,
        Some("gzip") => {
            let mut gz = GzDecoder::new(body.as_ref());
            let mut buffer: Vec<u8> = Vec::new();
            let decoded_size = gz.read_to_end(&mut buffer).unwrap();
            tracing::info!("gzip decoding. size: {}", decoded_size);
            String::from_utf8(buffer)?
        }
        Some(encoding) => {
            tracing::warn!("unknown content_encoding_type. type: {}", encoding);
            String::from_utf8(body.to_vec())?
        }
    };

    Ok(decoded)
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

pub async fn rtb_handler(
    uri: Uri,
    // Path(any): Path<String>,
    body_bytes: Bytes,
    method: Method,
    headers: HeaderMap,
    Extension(app_conf): Extension<Arc<AppConf>>,
) -> Response<Body> {
    let body = decode_body(&body_bytes, &headers).unwrap();
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
        match content_builder::build_content_with_replacing_macro(target_resource, &request) {
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
