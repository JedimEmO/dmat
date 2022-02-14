use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::from_stream;
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{
    ButtonContent, ButtonProps, ButtonType, CardProps, InteractiveListProps, ListEntry,
};
use dominator_material::utils::mixin::stream_handler_mixin;
use dominator_material::utils::signals::mutation::store_signal_value_opt_mixin;

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
            ()
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

    card!(
        CardProps::new()
            .with_title("Interactive list with selectable items", None)
            .body(list_body)
            .footer(html!("div", {
                .children(&mut[
                    toggle_button(&has_before, "Toggle Before"),
                    toggle_button(&has_after, "Toggle After"),
                ])
            })),
        |d| {
            d.class("demo-card")
                .apply(store_signal_value_opt_mixin(
                    from_stream(out.item_select_stream),
                    &selected_item,
                ))
                .apply(stream_handler_mixin(settings.to_stream(), move |_| {
                    entries.lock_mut().replace(vec![1, 2, 3])
                }))
        }
    )
}

fn dynamic_list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    card!(
        CardProps::new()
            .with_title("Dynamic list holding dom elements", None)
            .body(static_list!(vec![
                button!(ButtonProps {
                    content: Some(ButtonContent::Dom(text!("Add new entry"))),
                    click_handler: clone!(entries => move |_| {
                        entries.lock_mut().push_cloned("Hello!".into());
                    }),
                    button_type: ButtonType::Contained,
                    style: Default::default(),
                    disabled_signal: entries.signal_vec_cloned().len().map(|v| v >= 5)
                }),
                list!(entries
                    .signal_vec_cloned()
                    .map(|entry| html!("span", { .text(entry.as_str())}))),
            ])),
        |v| v.class("demo-card")
    )
}
