use std::{env, net::SocketAddr};

use axum::Router;
use tower_http::services::ServeDir;

pub fn two_serve_dirs() -> Router {
    // you can also have two `ServeDir`s nested at different paths
    let serve_dir_from_assets = ServeDir::new("assets");
    //let serve_dir_from_dist = ServeDir::new("dist");

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let assets_path = path.join("assets");

    Router::new().nest_service("/assets", serve_dir_from_assets)
    //.nest_service("/dist", serve_dir_from_dist)
}

pub async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    //tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    //.layer(TraceLayer::new_for_http())
}
