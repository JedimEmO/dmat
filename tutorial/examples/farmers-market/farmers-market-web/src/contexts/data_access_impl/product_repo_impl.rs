use crate::contexts::domain::product::{Product, ProductRepository};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use wasm_bindgen_futures::spawn_local;

pub struct ProductRepositoryImpl {
    pub client: reqwest::Client,
    url: String,
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn get_products(&self) -> anyhow::Result<MutableVec<Product>> {
        let (tx, rx) = futures::channel::oneshot::channel();

        let products = self.client.get(format!("{}/api/v1/products", self.url));

        spawn_local(async move {
            let products = products.send().await.unwrap().json().await;
            let _ = tx.send(products);
        });

        let products: Vec<farmers_market_api::Product> = rx.await??;

        Ok(MutableVec::new_with_values(
            products.into_iter().map(|p| p.into()).collect(),
        ))
    }
}

impl ProductRepositoryImpl {
    pub fn new(host: impl ToString) -> Self {
        Self {
            client: reqwest::Client::new(),
            url: format!("http://{}", host.to_string()),
        }
    }
}

impl From<farmers_market_api::Product> for Product {
    fn from(value: farmers_market_api::Product) -> Self {
        Self {
            name: Mutable::new(value.name),
            price: Mutable::new(value.price),
            quantity: Mutable::new(value.quantity),
        }
    }
}
