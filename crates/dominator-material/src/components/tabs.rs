use std::fmt::Debug;

use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

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

#[macro_export]
macro_rules! tabs {
    ($a: expr, $b: expr, $c: expr) => {{
        $crate::components::tabs::tabs($a, $b, $c, |d| d)
    }};

    ($a: expr, $b: expr, $c: expr, $mixin: expr) => {{
        $crate::components::tabs::tabs($a, $b, $c, $mixin)
    }};
}

#[inline]
pub fn tabs<
    TabList: SignalVec<Item = Tab<TabId>> + 'static,
    TabId: Clone + std::cmp::PartialEq + Debug + 'static,
    F,
>(
    current_tab: Mutable<TabId>,
    tabs_list: TabList,
    on_tab_change: Option<Rc<dyn Fn(TabId)>>,
    mixin: F,
) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("dmat-tabs")
        .apply(mixin)
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
