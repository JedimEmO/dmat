use crate::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use dmat_components::components::button::*;
use dmat_components::components::input::text_field::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;

pub fn counter() -> Dom {
    let counter_state = Mutable::new("0".to_string());

    html!("div", {
        .child(text_field!({
            .label(Some(html!("span", { .text("Counter Value")})))
            .value(MutableTValueAdapter::new_simple(&counter_state))
        }).0)
        .child(button!({
            .content(html!("span", { .text("Increment")}))
            .style(ButtonStyle::Prominent)
            .click_handler(clone!(counter_state => move |_| {
                counter_state.set((counter_state.get_cloned().as_str().parse::<i32>().unwrap() + 1).to_string());
            }))
            .apply(|dom_builder| dom_builder.attr("id", "increment-button"))
        }))
    })
}
