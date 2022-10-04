use std::rc::Rc;
use std::sync::Mutex;

use dominator::{clone, events, html, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::Signal;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;

#[macro_export]
macro_rules! interactive_list {
    ($props: expr) => {{
        $crate::components::interactive_list::interactive_list($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::interactive_list::interactive_list($props, $mixin)
    }};
}

pub struct ListEntry<TValue> {
    pub before: Option<Dom>,
    pub content: Dom,
    pub after: Option<Dom>,
    pub selected_signal: Box<dyn Signal<Item = bool> + Unpin>,
    pub item_value: TValue,
}

pub struct InteractiveListProps<TValue, TItems: SignalVec<Item = ListEntry<TValue>>> {
    pub items: TItems,
}

pub struct InteractiveListOut<TValue> {
    pub item_select_stream: Receiver<Option<TValue>>,
}

#[inline]
pub fn interactive_list<TValue, TItems, F>(
    props: InteractiveListProps<TValue, TItems>,
    mixin: F,
) -> (Dom, InteractiveListOut<TValue>)
where
    TValue: Copy + 'static,
    TItems: SignalVec<Item = ListEntry<TValue>> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (item_select_tx, item_select_stream) = channel(1);
    let item_select_tx = Rc::new(Mutex::new(item_select_tx));
    (
        html!("div", {
            .class("dmat-interactive-list")
            .apply(mixin)
            .children_signal_vec(props.items.map(clone!(item_select_tx => move |item| {
                let content = item.content;
                let item_value = item.item_value;
                let selected_signal = item.selected_signal;
                let before = item.before;
                let after = item.after;

                html!("div", {
                    .class("interactive-list-item")
                    .class_signal("-active", selected_signal)
                    .apply_if(before.is_some(), |d| d.class("-with-before"))
                    .apply_if(after.is_some(), |d| d.class("-with-after"))
                    .children(vec![
                        before.map(|v| html!("div", { .class("first").child(v)})),
                        Some(content),
                        after.map(|v| html!("div", { .class("last").child(v)})),
                    ].into_iter().flatten())
                    .apply(|d| {
                        d.event(clone!(item_select_tx => move |_: events::Click| {
                            let mut item_select_tx = item_select_tx.lock().unwrap_throw();
                            item_select_tx.try_send(Some(item_value)).unwrap_throw();
                        }))
                    })

                })
            })))
        }),
        InteractiveListOut { item_select_stream },
    )
}
