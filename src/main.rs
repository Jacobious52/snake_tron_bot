mod game;
mod server;

use opentelemetry::global;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use crate::server::router::router;

#[tokio::main]
async fn main() {
    // install optel with jaeger
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("snake_tron")
        .install_simple()
        //.install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = router();
    let addr = "127.0.0.1:8000";
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    global::shutdown_tracer_provider();
}
