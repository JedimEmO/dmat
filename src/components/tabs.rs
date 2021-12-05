use std::fmt::Debug;

use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use futures_util::StreamExt;
use wasm_bindgen::__rt::std::rc::Rc;

#[derive(Clone)]
pub enum TabContent {
    Label(String),
    NodeFn(Rc<dyn Fn() -> Dom>),
}

#[derive(Clone)]
pub struct Tab<TabId: Clone> {
    pub content: TabContent,
    pub id: TabId,
}

pub fn tabs<
    TabList: SignalVec<Item = Tab<TabId>> + 'static,
    TabId: Clone + std::cmp::PartialEq + Debug + 'static,
>(
    current_tab: Mutable<TabId>,
    tabs_list: TabList,
    on_tab_change: Option<Rc<dyn Fn(TabId)>>,
) -> Dom {
    html!("div", {
        .class("dmat-tabs")
        .children_signal_vec(tabs_list.map(clone!(current_tab, on_tab_change => move |v| {
            tab(&v, &current_tab, on_tab_change.clone())
        })))
    })
}

fn tab<TabId: Clone + std::cmp::PartialEq + Debug + 'static>(
    tab: &Tab<TabId>,
    meta: &Mutable<TabId>,
    select_cb: Option<Rc<dyn Fn(TabId)>>,
) -> Dom {
    let content_node = match &tab.content {
        TabContent::Label(label) => html!("span", {
            .text(label.as_str())
        }),
        TabContent::NodeFn(content) => content(),
    };

    html!("button", {
        .children(&mut [
            content_node,
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
                } else {
                    meta.set(tab.id.clone())
                }
            }
        }))
    })
}
