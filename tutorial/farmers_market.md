# Talking to a backend and managing state

So far, we have been working in isolation within the browser, without talking to any external system.
This is, of course, not how most applications work.

In this section, we'll look at how to talk to a backend, and how to manage state in our application.

## A simple backend

For our backend, we'll use a simple axum server hosting a small REST API.
It will serve the endpoint `/api/v1/products`, which returns a vector of products.

The interesting part of this, is that since both our frontend and backend are written in Rust, we can share the API type definitions between the two.

We can take a look at the API definition in `tutorial/examples/farmers-market/farmers-market-api/src/lib.rs`:

```rust
/// A purchasable product. Not available for purchase if quantity is 0.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}
```

Our handler function will simply return a vector of these products:

```rust
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
```

In the future, we would connect this handler to some sort of database, but for now, we'll just return a static vector of products, where the quantity of apples is randomized on each request.

To start this server, simply do `cargo run --bin farmers-market-backend`.
For the rest of this tutorial, you should keep the server running!

## Creating the frontend

To run the frontend application, go to `tutorial/examples/farmers-market/farmers-market-web` and run `npm install && npm start`.

For now, we want the farmers market UI to show a simple table of all the available products.
But we'll want to periodically fetch the list of products from the backend, and update the table accordingly whenever the list or the available quantity of a product changes!

Part of this is solved by using Mutable and MutableVec as we did in the counter example.
However, if we simply put the data in a MutableVec, and replace the entire vector whenever we get a new list of products, we'll force the entire table to re-render, which is not what we want.

This is where the `Updateable` trait from `crates/futures-signals-utils` and `crates/futures-signals-utils-derive` comes in.
It allows us to derive an `update(&self, &other)` function for a type.
This function will recursively apply changes (and only changes!) to our data, causing the DOM to only update the parts that have changed.

The `Updateable` trait is also implemented for `Mutable` and `MutableVec`, when they hold types that are `Clone + PartialEq`, so we can use it to update our data structures.

Let's create a `Product` struct for the frontend data model:

```rust
#[derive(Updateable, Clone)]
pub struct Product {
    pub name: Mutable<String>,
    pub price: Mutable<f64>,
    pub quantity: Mutable<u32>,
}
```

As we can see, we're using `Mutable` for all the fields, so that we can update them later and get signals that will let us know when they change.
 
It's also useful to create an interface for interacting with the backend, so that we can easily swap out the backend for a fake one in tests.
We'll create the `ProductRepository` trait for this:

```rust
#[async_trait]
pub trait ProductRepository {
    async fn get_products(&self) -> anyhow::Result<MutableVec<Product>>;
}
```

The implementation of this repository will simply fetch the list of products from the backend, and convert them to our frontend data model:

```rust
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
```

This uses a `reqwest` client to perform the get request, and returns the response as a vec of the `Product` type we defined in the API crate.

We then utilise an impl of `From<farmers_market_api::Product>` for `Product` to convert the API type to our frontend type with `products.into_iter().map(|p| p.into()).collect()`:

```rust
impl From<farmers_market_api::Product> for Product {
    fn from(value: farmers_market_api::Product) -> Self {
        Self {
            name: Mutable::new(value.name),
            price: Mutable::new(value.price),
            quantity: Mutable::new(value.quantity),
        }
    }
}
```

We can now create a small async loop that will periodically fetch the list of products from the backend, and update our MutableVec with the new list:

```rust
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
```

Observe that `products.update(new_products);` is using the `Updateable` trait to sparsely update the MutableVec of products.
The future performing the loop will have to be executed somehow; in the example app it is simply spawned using `wasm_bindgen_futures::spawn_local`

We also leak the `ProductService` struct.
This gives us a `&'static` lifetime reference to the service, which makes sure it will live as long as the application.
This makes it a bit easier to pass around the service to the different parts of the application, but you could also use a `Rc` or `Arc` if you prefer.

Finally, we can create our table:

```rust
pub fn product_list(products: impl SignalVec<Item = Product> + 'static) -> Dom {
    table!({
        .headers([
            html!("span", {
                .text("Name")
            }),
            html!("span", {
                .text("Price")
            }),
            html!("span", {
                .text("Quantity")
            }),
        ])
        .rows_signal_vec(products.map(|product| {
            html!("tr", {
                .children(&mut [
                    html!("td", {
                        .text_signal(product.name.signal_cloned())
                    }),
                    html!("td", {
                        .text_signal(product.price.signal_ref(|price| format!("${:.2}", price)))
                    }),
                    html!("td", {
                        .text_signal(product.quantity.signal_ref(|quantity| format!("{} in stock", quantity)))
                    }),
                ])
            })
        }))
    })
}
```

If you run this UI while the backend is running, you should see a table with the products from the backend, and the quantity of apples should change every second!
And if you inspect the DOM, you'll see that only the quantity of apples is updated every second, and the rest of the table is left untouched.

It's also worth noting that the `product_list` function is depending on strictly the parameters and types that it needs to perform its job, nothing else.
It would certainly be possible to pass in at `&'static ProductService` instead of a `impl SignalVec<Item = Product>`, but that would make it harder to test, and it would be harder to see what the function actually depends on.
We know, by looking at the function signature, that this component is not mutating the product vector!

----
Previous: [Lightweight Dmat](./lightweight_dmat.md) 
