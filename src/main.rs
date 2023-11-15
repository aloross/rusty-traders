use std::net::SocketAddr;

use crate::partials::hello;
use axum::{routing::get, Router};
use space_traders::{agent, utils};
use tower_http::services::ServeDir;

pub mod partials;
pub mod space_traders;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(hello::to_html))
        .route("/status", get(utils::get_status))
        .route("/agent", get(agent::get_agent));

    serve(app, 3030).await
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
