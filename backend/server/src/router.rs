use crate::AppState;
use axum::{routing::get, Router};


pub fn new_router(state: AppState) -> axum::Router {
    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root page"} ));
    // check if user is logged in
    let authorized_routes = Router::new();

    Router::new()
    .merge(public_routes)
    .merge(authorized_routes)
}