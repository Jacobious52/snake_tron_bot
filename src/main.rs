mod game;
mod server;

use crate::server::router::router;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "snake_tron_bot=info,tower_http=info")
    }
    tracing_subscriber::fmt::init();

    let app = router();
    let addr = "127.0.0.1:8000";
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
