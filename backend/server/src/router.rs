use crate::{handler::{with_handler, with_public_handler}, AppState};
use axum::{http::HeaderValue, routing::{get, post}, Router};
use hyper::{header::CONTENT_TYPE, Method};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, OnResponse, TraceLayer}, LatencyUnit};
use tracing::Level;
use uchat_endpoint::{post::endpoint::{Bookmark, NewPost, Boost, TrendingPosts, React}, user::endpoint::{CreateUser, Login}, Endpoint};



pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root page"} ))
        .route(
        CreateUser::URL,
            post(with_public_handler::<CreateUser>))
        .route(
            Login::URL,
            post(with_public_handler::<Login>));
    // check if user is logged in
    let authorized_routes = Router::new()
        .route(NewPost::URL, post(with_handler::<NewPost>))
        .route(Bookmark::URL, post(with_handler::<Bookmark>))
        .route(Boost::URL, post(with_handler::<Boost>))

        .route(React::URL, post(with_handler::<React>))


        .route(TrendingPosts::URL, post(with_handler::<TrendingPosts>));


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
                        .latency_unit(LatencyUnit::Micros)
                    ),
                )
                .layer(
                    // CORS layer allows comms between FE and BE
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_credentials(true)
                        .allow_origin(
                            std::env::var("FRONTEND_URL")
                            .unwrap()
                            .parse::<HeaderValue>()
                            .unwrap()
                        )
                        .allow_headers([CONTENT_TYPE])
                )
                .layer(
                    axum::Extension(state.clone()),
                )
        )
        .with_state(state)
}