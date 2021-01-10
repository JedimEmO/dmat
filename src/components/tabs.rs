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
    on_tab_change: Option<Rc<dyn Fn(TabId)>>,
}

impl<TabId: Clone + std::cmp::PartialEq + 'static> Tabs<TabId> {
    pub fn new(current_tab: Mutable<TabId>) -> Tabs<TabId> {
        Tabs {
            current_tab,
            on_tab_change: None,
        }
    }

    pub fn on_tab_select<F: 'static>(mut self, change_listener: F) -> Self
    where
        F: Fn(TabId),
    {
        self.on_tab_change = Some(Rc::new(change_listener));
        self
    }

    pub fn build_static(self, tabs: Vec<Tab<TabId>>) -> Dom {
        let state = Rc::new(self);

        Dom::with_state(state, |state| {
            html!("div", {
                .class("dmat-tabs")
                .children(tabs.iter().map(clone!(state => move |v| {
                    tab(v, &state.current_tab, state.on_tab_change.clone())
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
                    tab(&v, &state.current_tab, state.on_tab_change.clone())
                })))
            })
        })
    }
}

fn tab<TabId: Clone + std::cmp::PartialEq + 'static>(
    tab: &Tab<TabId>,
    meta: &Mutable<TabId>,
    select_cb: Option<Rc<dyn Fn(TabId)>>,
) -> Dom {
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
        .event(clone!(meta, tab, select_cb => move |_: events::Click| {
            let active = meta.get_cloned();

            if active != tab.id {
                if let Some(cb) = &select_cb {
                    cb(tab.id.clone())
                }
            }
        }))
    })
}
