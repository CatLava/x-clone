use crate::AppState;
use axum::{http::HeaderValue, routing::get, Router};
use hyper::{header::CONTENT_TYPE, Method};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, OnResponse, TraceLayer}, LatencyUnit};
use tracing::Level;


pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root page"} ));
    // check if user is logged in
    let authorized_routes = Router::new();

    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
        .layer(
            // arrange the layers properly
            ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros)),
            ),
        
        )
        .layer(
            // CORS layer allows comms between FE and BE
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PATCH, Method::PUT])
                .allow_credentials(true)
                .allow_origin(
                    std::env::var("FRONTEND_URL")
                    .unwrap()
                    .parse::<HeaderValue>()
                    .unwrap()
                )
                .allow_headers([CONTENT_TYPE])
        )
}