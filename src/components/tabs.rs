use dominator::{clone, Dom, events, html};
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::{MutableVec, SignalVec};
use futures_signals::signal_vec::SignalVecExt;
use futures_util::StreamExt;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub struct Tab<TabId: Clone> {
    pub label: String,
    pub id: TabId,
}

struct TabExternal<TabId: Clone> {
    current_active: Mutable<Option<TabId>>
}

enum TabMeta<TabId: Clone> {
    Owned(Mutable<Option<TabId>>),
    External(TabExternal<TabId>),
}

impl<TabId: Clone> TabMeta<TabId> {
    fn read_active(&self) -> ReadOnlyMutable<Option<TabId>> {
        match self {
            TabMeta::Owned(m) => m.read_only(),
            TabMeta::External(ext) => ext.current_active.read_only()
        }
    }

    fn active_mutable(&self) -> Mutable<Option<TabId>> {
        match self {
            TabMeta::External(_ext) => Mutable::new(None),
            TabMeta::Owned(owned) => owned.clone()
        }
    }
}

pub struct Tabs<TabId: Clone> {
    tab_meta: TabMeta<TabId>,
    on_tab_change: Option<Rc<dyn Fn(Option<TabId>)>>,
}

impl<TabId: Clone + std::cmp::PartialEq + 'static> Tabs<TabId> {
    pub fn new() -> Tabs<TabId> {
        Tabs {
            tab_meta: TabMeta::Owned(Mutable::new(None)),
            on_tab_change: None,
        }
    }

    pub fn initial_active_tab_id(self, id: Option<TabId>) -> Self {
        self.tab_meta.active_mutable().set(id);
        self
    }

    pub fn on_tab_change<F: 'static>(mut self, change_listener: F) -> Self
        where F: Fn(Option<TabId>) {
        self.on_tab_change = Some(Rc::new(change_listener));
        self
    }

    pub fn build_static(mut self, tabs: Vec<Tab<TabId>>) -> Dom {
        let state = Rc::new(self);

        Dom::with_state(state, |state| {
            let mut active_stream = state.tab_meta.read_active().signal_cloned().to_stream();

            html!("div", {
                .future(clone!(state => async move {
                    loop {
                        if let Some(tab) = active_stream.next().await {
                            if let Some(cb) = &state.on_tab_change {
                                cb(tab);
                            }
                        }
                    }
                }))
                .class("dmat-tabs")
                .children(tabs.iter().map(clone!(state => move |v| {
                    tab(v, &state.tab_meta)
                })))
            })
        })
    }

    pub fn build_dynamic<B>(mut self, tabs: B) -> Dom
        where B: SignalVec<Item=Tab<TabId>> + 'static {
        let state = Rc::new(self);

        Dom::with_state(state, move |state| {
            let mut active_stream = state.tab_meta.read_active().signal_cloned().to_stream();

            html!("div", {
                .future(clone!(state => async move {
                    loop {
                        if let Some(tab) = active_stream.next().await {
                            if let Some(cb) = &state.on_tab_change {
                                cb(tab);
                            }
                        }
                    }
                }))
                .class("dmat-tabs")
                .children_signal_vec(tabs.map(clone!(state => move |v| {
                    tab(&v, &state.tab_meta)
                })))
            })
        })
    }
}

fn tab<TabId: Clone + std::cmp::PartialEq + 'static>(tab: &Tab<TabId>, meta: &TabMeta<TabId>) -> Dom {
    let active = meta.read_active();
    let set_active = meta.active_mutable();

    html!("button", {
        .children(&mut [
            html!("span", { .text(tab.label.as_str())}),
            html!("span", {
                .class("dmat-tab-indicator")
            })
        ])
        .class("tab")
        .class_signal("active", meta.read_active().signal_cloned().map(clone!(tab => move |e| {
                match e {
                    Some(inc) => {
                        tab.id == inc
                    },
                    _ => false
                }
            }))
        )
        .event(clone!(active, set_active, tab => move |_: events::Click| {
            if active.get_cloned() != Some(tab.id.clone()) {
                set_active.set_neq(Some(tab.id.clone()));
            }
        }))
    })
}
