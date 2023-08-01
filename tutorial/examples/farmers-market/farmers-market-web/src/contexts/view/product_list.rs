use crate::contexts::domain::product::Product;
use dmat_components::components::*;
use dominator::{html, Dom};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

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
