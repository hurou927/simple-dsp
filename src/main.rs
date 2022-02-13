mod app_conf;
mod arg_option;
mod handlers;
mod resource_selector;
mod rtb_model;

use crate::app_conf::read_app_conf;
use axum::{routing::any, AddExtensionLayer, Router};
use clap::StructOpt;
use std::net::SocketAddr;
use std::{error::Error, sync::Arc};
use tracing::Level;

fn subscribe_tracing() {
    let file_appender = tracing_appender::rolling::never("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let collector = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(collector).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = arg_option::Args::parse();

    subscribe_tracing();
    tracing::info!("args: {:?}", args);

    let app_conf = read_app_conf(&args.conf_path)?;

    for r in app_conf.resources.iter() {
        println!("path: {}, imp_condition: {:?}", r.uri, r.imp_condition);
    }

    // build our application with a route
    let app = Router::new()
        .fallback(any(handlers::handler))
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
