use super::routes::tick;
use super::{model::Games, routes::debug};
use axum::handler::get;
use axum::{body::Body, handler::post, routing::BoxRoute, AddExtensionLayer, Router};
use http::{header, HeaderValue};
use std::sync::{Arc, Mutex};
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder,
};
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};

pub fn router() -> Router<BoxRoute> {
    let cors_middleware = cors();

    let games = Arc::new(Mutex::new(Games::new()));

    Router::new()
        .route("/", post(tick).options(|| async { "" }))
        .route("/debug", get(debug))
        .layer(cors_middleware)
        .layer(AddExtensionLayer::new(games))
        .layer(TraceLayer::new_for_http())
        .boxed()
}

fn cors() -> Stack<
    SetResponseHeaderLayer<HeaderValue, Body>,
    Stack<
        SetResponseHeaderLayer<HeaderValue, Body>,
        Stack<SetResponseHeaderLayer<HeaderValue, Body>, Identity>,
    >,
> {
    ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("OPTION, GET, POST, PATCH, DELETE"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("access-control-allow-origin, content-type"),
        ))
        .into_inner()
}
