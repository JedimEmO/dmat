use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::Signal;
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use std::cell::RefCell;
use std::fmt::Debug;
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

pub type TabChangeCallbackFnMut<TabId> = Rc<RefCell<dyn FnMut(TabId)>>;

#[inline]
pub fn tabs<TabList, TabId, TActiveSignal, FActiveFn, F>(
    active_tab_signal_factory: FActiveFn,
    tabs_list: TabList,
    on_tab_change: Option<TabChangeCallbackFnMut<TabId>>,
    mixin: F,
) -> Dom
where
    TabList: SignalVec<Item = Tab<TabId>> + 'static,
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    FActiveFn: Fn(TabId) -> TActiveSignal + 'static,
    TActiveSignal: Signal<Item = bool> + 'static,
{
    html!("div", {
        .class("dmat-tabs")
        .apply(mixin)
        .children_signal_vec(tabs_list.map(move |v| {
            tab(&v, active_tab_signal_factory(v.id), on_tab_change.clone())
        }))
    })
}

fn tab<
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    TIsActiveSignal: Signal<Item = bool> + 'static,
>(
    tab: &Tab<TabId>,
    is_active: TIsActiveSignal,
    select_cb: Option<TabChangeCallbackFnMut<TabId>>,
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
        .class_signal("active", is_active)
        .event(clone!(tab, select_cb => move |_: events::Click| {
            if let Some(cb) = &select_cb {
                cb.borrow_mut()(tab.id)
            }
        }))
    })
}
