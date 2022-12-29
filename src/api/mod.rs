use std::io::Result;
use std::net::SocketAddr;

use axum::{ 
    extract::Extension, Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::Context;

pub mod routes;

fn api_router() -> Router {
    Router::new()
        .merge(routes::oauth::router())
}

pub async fn serve(context: Context) -> Result<()> {
    let server_port = context.config.server_port;

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(context)),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], server_port));
    log::info!("serving api on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}