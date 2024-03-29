mod app_conf;
mod arg_option;
mod content_builder;
mod handlers;
mod rtb_model;

use crate::app_conf::read_app_conf;
use axum::Extension;
use axum::{routing::any, Router};
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::{error::Error, sync::Arc};
use tracing::Level;

static LOG_DIR: &str = "logs";
static LOG_FILENAME: &str = "app.log";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_option::Args::parse();

    let file_appender = tracing_appender::rolling::never(LOG_DIR, LOG_FILENAME);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();

    tracing::info!("args: {:?}", args);
    let conf_path = shellexpand::tilde(&args.conf_path).to_string();
    let app_conf = read_app_conf(&PathBuf::from(conf_path))?;

    for r in app_conf.resources.iter() {
        println!("path: {}, imp_condition: {:?}", r.uri, r.imp_condition);
    }

    // build our application with a route
    let app = Router::new()
        .fallback(any(handlers::rtb_handler))
        .layer(Extension(Arc::new(app_conf.clone())));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
