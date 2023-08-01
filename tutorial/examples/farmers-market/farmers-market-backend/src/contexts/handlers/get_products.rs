use axum::response::Json;
use farmers_market_api::Product;

pub async fn get_products() -> Json<Vec<Product>> {
    vec![
        Product {
            id: Default::default(),
            name: "fresh carrots!".to_string(),
            price: 10.0,
            quantity: 5,
        },
        Product {
            id: Default::default(),
            name: "fresh apples!".to_string(),
            price: 5.0,
            quantity: rand::random::<u32>() % 10,
        },
    ]
    .into()
}
