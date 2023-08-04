use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::__rt::std::rc::Rc;

use dmat_components::components::*;
use dmat_components::utils::mixin::stream_handler_mixin;

use crate::utils::toggle_button::toggle_button;

pub fn list_demo() -> Dom {
    container!({ .children([dynamic_list_demo(), interactive_list_demo(),]) })
}

fn interactive_list_demo() -> Dom {
    let selected_item = Mutable::<Vec<usize>>::new(vec![]);
    let entries = MutableVec::new_with_values(vec![1, 2, 3]);

    let has_before = Mutable::new(false);
    let has_after = Mutable::new(false);

    let settings = map_ref! {
        let _before = has_before.signal_cloned(),
        let _after = has_after.signal_cloned() => move {
        }
    };

    let items = entries
        .signal_vec()
        .map(clone!(has_before, has_after => move |entry| ListEntry {
            before: match has_before.get() {
                true => Some(html!("span", { .text("Before")})),
                _ => None
            },
            content: html!("div", { .text(format!("Entry {}", entry).as_str())}),
            after: match has_after.get() {
                true => Some(html!("span", { .text("After")})),
                _ => None
            }
        }));

    card!({
        .child(content_block!({
            .title_section(Some(title!( {
                .header_text("Interactive list with selectable items".to_string())
            })))
            .media_section(Some(container!({
                .children([
                    list!({
                        .items([
                            interactive_list!({
                                .items_signal_vec(items)
                                .selected_indexes_signal(selected_item.signal_cloned())
                                .on_item_selected(move |idx| {
                                    selected_item.set(vec![idx]);
                                })
                            }),
                            toggle_button(&has_before, "Toggle Before"),
                            toggle_button(&has_after, "Toggle After")
                        ])
                    })
                ])
            })))
        }))
        .apply(move |d| {
            d.class("demo-card")
                .apply(stream_handler_mixin(settings.to_stream(), move |_| {
                    entries.lock_mut().replace(vec![1, 2, 3])
                }))
        })
    })
}

fn dynamic_list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    card!({
        .child(content_block!({
            .title_section(Some(title!({
                .header_text("Dynamic list holding dom elements".to_string())
            })))
            .media_section(Some(list!({
                .items([
                    button!({
                        .label("Add new entry")
                        .click_handler(clone!(entries => move |_| {
                            entries.lock_mut().push_cloned("Hello!".into());
                        }))
                        .disabled_signal(entries.signal_vec_cloned().len().map(|v| v >= 5))
                    }),
                    list!({
                        .items_signal_vec(entries
                        .signal_vec_cloned()
                        .map(|entry| html!("span", { .text(entry.as_str())})))
                    }),
                ])
            })))
        }))
        .apply(|v| v.class("demo-card"))
    })
}
