use dominator::{events, clone, Dom, html};
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal::SignalExt;
use futures_util::StreamExt;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub struct Tab {
    pub label: String,
    pub id: usize,
}

struct TabExternal {
    current_active: Mutable<Option<usize>>
}

enum TabMeta {
    Owned(Mutable<Option<usize>>),
    External(TabExternal),
}

impl TabMeta {
    fn read_active(&self) -> ReadOnlyMutable<Option<usize>> {
        match self {
            TabMeta::Owned(m) => m.read_only(),
            TabMeta::External(ext) => ext.current_active.read_only()
        }
    }

    fn active_mutable(&self) -> Mutable<Option<usize>> {
        match self {
            TabMeta::External(_ext) => Mutable::new(None),
            TabMeta::Owned(owned) => owned.clone()
        }
    }
}

enum TabData {
    Static(Vec<Tab>),
    Dynamic(Rc<MutableVec<Tab>>),
}


pub struct Tabs {
    tab_meta: TabMeta,
    tab_data: TabData,
    on_tab_change: Option<Rc<dyn Fn(Option<usize>)>>,
}

impl Tabs {
    pub fn build() -> Tabs {
        Tabs {
            tab_data: TabData::Static(vec![]),
            tab_meta: TabMeta::Owned(Mutable::new(None)),
            on_tab_change: None,
        }
    }

    pub fn active_tab_id(self, id: Option<usize>) -> Self {
        self.tab_meta.active_mutable().set(id);
        self
    }

    pub fn on_tab_change<F: 'static>(mut self, change_listener: F) -> Self
        where F: Fn(Option<usize>) {
        self.on_tab_change = Some(Rc::new(change_listener));
        self
    }

    pub fn static_tabs(mut self, tabs: Vec<Tab>) -> Self {
        self.tab_data = TabData::Static(tabs);
        self
    }

    pub fn dynamic_tabs(mut self, tabs: Rc<MutableVec<Tab>>) -> Self {
        self.tab_data = TabData::Dynamic(tabs);
        self
    }

    pub fn dom(self) -> Dom {
        tabs(self)
    }
}

fn tabs(tabs: Tabs) -> Dom {
    Dom::with_state(tabs, |tabs| {
        match &tabs.tab_data {
            TabData::Dynamic(dyn_data) => dynamic_tabs(dyn_data, &tabs.tab_meta),
            TabData::Static(data) => static_tabs(data, &tabs.tab_meta, &tabs.on_tab_change)
        }
    })
}

fn dynamic_tabs(_tabs: &Rc<MutableVec<Tab>>, _meta: &TabMeta) -> Dom {
    html!("div")
}

fn static_tabs(tabs: &Vec<Tab>, meta: &TabMeta, on_tab_change: &Option<Rc<dyn Fn(Option<usize>)>>) -> Dom {
    let mut active_stream = meta.read_active().signal().to_stream();

    html!("div", {
        .future(clone!(on_tab_change => async move {
            loop {
                if let Some(tab) = active_stream.next().await {
                    if let Some(cb) = &on_tab_change {
                        cb(tab);
                    }
                }
            }
        }))
        .class("tabs")
        .children(tabs.iter().map(|v| {
            tab(v, meta)
        }))
    })
}

fn tab(tab: &Tab, meta: &TabMeta) -> Dom {
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
            if active.get() != Some(tab.id) {
                set_active.set_neq(Some(tab.id));
            }
        }))
    })
}
