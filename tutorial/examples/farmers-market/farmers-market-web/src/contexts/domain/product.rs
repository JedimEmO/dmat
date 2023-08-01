use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use futures_signals_utils::Updateable;
use futures_signals_utils_derive::Updateable;
use std::future::Future;

#[async_trait]
pub trait ProductRepository {
    async fn get_products(&self) -> anyhow::Result<MutableVec<Product>>;
}

#[derive(Updateable, Clone)]
pub struct Product {
    pub name: Mutable<String>,
    pub price: Mutable<f64>,
    pub quantity: Mutable<u32>,
}

pub struct ProductService {
    pub products: MutableVec<Product>,
}

pub fn run_product_service(
    product_repository: impl ProductRepository,
) -> (impl Future<Output = ()>, &'static ProductService) {
    let product_service = ProductService::new();
    let products = product_service.products.clone();

    let future = async move {
        loop {
            if let Ok(new_products) = product_repository.get_products().await {
                products.update(new_products);
            }

            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        }
    };

    let leaked_product_service = Box::leak(Box::new(product_service));
    (future, leaked_product_service)
}

impl ProductService {
    fn new() -> Self {
        Self {
            products: MutableVec::new(),
        }
    }
}
