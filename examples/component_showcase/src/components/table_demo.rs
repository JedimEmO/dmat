use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use dmat_components::components::input::value_adapters::mutable_t_value_adapter::MutableTValueAdapter;
use dmat_components::components::input::*;
use dmat_components::components::*;
use dmat_components::utils::timeout::timeout;

pub fn table_demo() -> Dom {
    let is_loading = Mutable::new(false);

    let controls = list!({
        .items(vec![
            button!({
                .label("Toggle loading")
                .disabled_signal(is_loading.signal())
                .click_handler(clone!(is_loading => move |_| {
                    is_loading.set(true);
                    timeout(clone!(is_loading => move || {
                        is_loading.set(false);
                    }), std::time::Duration::from_secs(1));
                }))
            })
        ])
    });

    let data = MutableVec::new_with_values(vec![
        Mutable::new("bob".to_string()),
        Mutable::new("alice".to_string()),
        Mutable::new("eve".to_string()),
    ]);
    let rows = data.signal_vec_cloned().map(|v| {
        html!("tr", {
            .children(&mut[
                html!("td", {
                    .style("width", "300px")
                    .child(text_field!({
                        .value(MutableTValueAdapter::new_simple(&v))
                    }))
                }),
                html!("td", {
                    .text_signal(v.signal_ref(|v| v.len().to_string()))
                }),
            ])
        })
    });

    let t = table!({
        .headers(vec![
            html!("span", {
                .text("Name")
            }),
            html!("span", {
                .text("Name length")
            }),
        ])
        .rows_signal_vec(rows)
        .is_loading_signal(is_loading.signal())
    });

    container!({.apply(|d| d.child(content_block!({
        .title_section(Some(title!({
            .header_text("Simple table".to_string())
        })))
        .media_section(Some(list!({
            .items(vec![
                controls,
                t,
            ])
        })))
        .apply(|v| v.class("demo-card"))
    })))})
}
