use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Router;
use div_tags_rust::chart;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest("/chart", chart::build_router());

    // run it with hyper on localhost:3000
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
