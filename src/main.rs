use axum::Router;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .merge(routes::upload_handler::upload_routes());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
