use dominator::{clone, Dom, html};
use futures_signals::signal::Mutable;
use dmat_components::components::ButtonStyle;

pub fn counter() -> Dom {
    let counter_state = Mutable::new("0".to_string());

    html!("div", {
        .child(text_field!({
            .label("Counter Value")
            .value(counter_state.clone())
            .disabled()
        }).0)
        .child(button!({
            .label("Increment")
            .style(ButtonStyle::Prominent)
            .click_handler(clone!(counter_state => move |_| {
                counter_state.set((i32::from_str_radix(counter_state.get_cloned().as_str(), 10).unwrap() + 1).to_string());
            }))
            .apply(|dom_builder| dom_builder.attr("id", "increment-button"))
        }))
    })
}