use dominator::{clone, events, html, Dom};
use futures::channel::mpsc::Receiver;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

pub struct ListEntry {
    pub before: Option<Dom>,
    pub content: Dom,
    pub after: Option<Dom>,
}

pub struct InteractiveListOut<TValue> {
    pub item_select_stream: Receiver<Option<TValue>>,
}

#[component(render_fn = interactive_list)]
pub struct InteractiveList<TOnItemSelected: Fn(usize) = fn(usize) -> ()> {
    #[signal_vec]
    #[default(vec![])]
    pub items: ListEntry,

    #[signal]
    #[default(vec![])]
    pub selected_indexes: Vec<usize>,

    #[default(|_| {})]
    pub on_item_selected: TOnItemSelected,
}

#[inline]
pub fn interactive_list(props: impl InteractiveListPropsTrait + 'static) -> Dom {
    let InteractiveListProps {
        items,
        selected_indexes,
        on_item_selected,
        apply,
    } = props.take();

    let on_item_selected = Rc::new(on_item_selected);
    let selected_bc = selected_indexes.broadcast();

    html!("div", {
        .class("dmat-interactive-list")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap_throw()))
        .children_signal_vec(items.enumerate().map(clone!(on_item_selected => move |(idx, item)| {
            let idx = idx.get().unwrap_throw();
            let content = item.content;
            let before = item.before;
            let after = item.after;
            let is_selected = selected_bc.signal_cloned().map(move |selected_indexes| {
                selected_indexes.contains(&idx)
            });

            html!("div", {
                .class("interactive-list-item")
                .class_signal("-active", is_selected)
                .apply_if(before.is_some(), |d| d.class("-with-before"))
                .apply_if(after.is_some(), |d| d.class("-with-after"))
                .children(vec![
                    before.map(|v| html!("div", { .class("first").child(v)})),
                    Some(content),
                    after.map(|v| html!("div", { .class("last").child(v)})),
                ].into_iter().flatten())
                .apply(|d| {
                    d.event(clone!(on_item_selected => move |_: events::Click| {
                        (*on_item_selected)(idx);
                    }))
                })

            })
        })))
    })
}
