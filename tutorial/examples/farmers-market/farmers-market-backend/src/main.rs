mod contexts;

use crate::contexts::handlers::get_products::get_products;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1/products", get(get_products));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
