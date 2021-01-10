use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use futures_util::StreamExt;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub struct Tab<TabId: Clone> {
    pub label: String,
    pub id: TabId,
}

pub struct Tabs<TabId: Clone> {
    current_tab: Mutable<TabId>,
}

impl<TabId: Clone + std::cmp::PartialEq + 'static> Tabs<TabId> {
    pub fn new(current_tab: Mutable<TabId>) -> Tabs<TabId> {
        Tabs { current_tab }
    }

    pub fn build_static(self, tabs: Vec<Tab<TabId>>) -> Dom {
        let state = Rc::new(self);

        Dom::with_state(state, |state| {
            html!("div", {
                .class("dmat-tabs")
                .children(tabs.iter().map(clone!(state => move |v| {
                    tab(v, &state.current_tab)
                })))
            })
        })
    }

    pub fn build_dynamic<B>(self, tabs: B) -> Dom
    where
        B: SignalVec<Item = Tab<TabId>> + 'static,
    {
        let state = Rc::new(self);

        Dom::with_state(state, move |state| {
            html!("div", {
                .class("dmat-tabs")
                .children_signal_vec(tabs.map(clone!(state => move |v| {
                    tab(&v, &state.current_tab)
                })))
            })
        })
    }
}

fn tab<TabId: Clone + std::cmp::PartialEq + 'static>(
    tab: &Tab<TabId>,
    meta: &Mutable<TabId>,
) -> Dom {
    let active = meta.get_cloned();

    html!("button", {
        .children(&mut [
            html!("span", { .text(tab.label.as_str())}),
            html!("span", {
                .class("dmat-tab-indicator")
            })
        ])
        .class("tab")
        .class_signal("active", meta.signal_cloned().map(clone!(tab => move |e| {
                tab.id == e
            }))
        )
        .event(clone!(active, meta, tab => move |_: events::Click| {
            if active != tab.id.clone() {
                meta.set_neq(tab.id.clone());
            }
        }))
    })
}
