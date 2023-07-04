use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::always;
use futures_signals::signal::from_stream;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::__rt::std::rc::Rc;

use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::TitleProps;
use dmat_components::components::{InteractiveListProps, ListEntry};
use dmat_components::utils::mixin::stream_handler_mixin;
use dmat_components::utils::signals::mutation::store_signal_value_opt_mixin;

use crate::utils::toggle_button::toggle_button;

pub fn list_demo() -> Dom {
    container!(|d| { d.children(&mut [dynamic_list_demo(), interactive_list_demo(),]) })
}

fn interactive_list_demo() -> Dom {
    let selected_item = Mutable::<Option<u32>>::new(None);
    let entries = MutableVec::new_with_values(vec![1, 2, 3]);

    let has_before = Mutable::new(false);
    let has_after = Mutable::new(false);

    let settings = map_ref! {
        let _before = has_before.signal_cloned(),
        let _after = has_after.signal_cloned() => move {

        }
    };

    let items = entries.signal_vec().map(
        clone!(selected_item, has_before, has_after => move |entry| ListEntry {
            before: match has_before.get() {
                true => Some(text!("Before")),
                _ => None
            },
            content: html!("div", { .text(format!("Entry {}", entry).as_str())}),
            after: match has_after.get() {
                true => Some(text!("After")),
                _ => None
            },
            selected_signal: Box::new(
                selected_item.signal_ref(clone!(entry => move |v| v == &Some(entry))),
            ),
            item_value: entry
        }),
    );

    let props = InteractiveListProps { items };

    let (list_body, out) = interactive_list!(props);

    card!({
        .child(content_block!(ContentBlockProps {
            title_section: Some(title!(TitleProps {
                header_text_signal: always("Interactive list with selectable items".to_string()),
                sub_header_text_signal: always(None)
            })),
            media_section: Some(container!(|d| d.child(list!({
                .rows([
                    list_body,
                    toggle_button(&has_before, "Toggle Before"),
                    toggle_button(&has_after, "Toggle After")
                ])
            })))),
            supporting_section: None,
            footer_section: None
        }))
        .apply(move |d| {
            d.class("demo-card")
                .apply(store_signal_value_opt_mixin(
                    from_stream(out.item_select_stream),
                    &selected_item,
                ))
                .apply(stream_handler_mixin(settings.to_stream(), move |_| {
                    entries.lock_mut().replace(vec![1, 2, 3])
                }))
        })
    })
}

fn dynamic_list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    card!({
        .child(content_block!(ContentBlockProps {
            title_section: Some(title!(TitleProps {
                header_text_signal: always("Dynamic list holding dom elements".to_string()),
                sub_header_text_signal: always(None)
            })),
            media_section: Some(list!({
                .rows([
                    button!({
                        .label("Add new entry")
                        .click_handler(clone!(entries => move |_| {
                            entries.lock_mut().push_cloned("Hello!".into());
                        }))
                        .disabled_signal(entries.signal_vec_cloned().len().map(|v| v >= 5))
                    }),
                    list!({
                        .rows_signal_vec(entries
                        .signal_vec_cloned()
                        .map(|entry| html!("span", { .text(entry.as_str())})))
                    }),
                ])
            })),
            supporting_section: None,
            footer_section: None
        }))
        .apply(|v| v.class("demo-card"))
    })
}
