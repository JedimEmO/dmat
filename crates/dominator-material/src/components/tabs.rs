use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::signal::Signal;
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use std::cell::RefCell;
use std::fmt::Debug;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::HtmlElement;

pub struct TabsProps<
    TabList: SignalVec<Item = TabId> + 'static,
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    ActiveFn: 'static,
    ActiveSignal: 'static,
    TabFn: Fn(TabId) -> Dom + 'static,
> where
    ActiveFn: Fn(TabId) -> ActiveSignal,
    ActiveSignal: Signal<Item = bool>,
{
    pub tab_fn: TabFn,
    pub active_tab_signal_factory: ActiveFn,
    pub tabs_list: TabList,
    pub on_tab_change: Option<TabChangeCallbackFnMut<TabId>>,
}

#[macro_export]
macro_rules! tabs {
    ($props: expr) => {{
        $crate::components::tabs::tabs($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::tabs::tabs($props, $mixin)
    }};
}

pub type TabChangeCallbackFnMut<TabId> = Rc<RefCell<dyn FnMut(TabId)>>;

#[inline]
pub fn tabs<TabList, TabId, TActiveSignal, ActiveFn, F, TabFn>(
    props: TabsProps<TabList, TabId, ActiveFn, TActiveSignal, TabFn>,
    mixin: F,
) -> Dom
where
    TabList: SignalVec<Item = TabId> + 'static,
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    ActiveFn: Fn(TabId) -> TActiveSignal + 'static,
    TActiveSignal: Signal<Item = bool> + 'static,
    TabFn: Fn(TabId) -> Dom + 'static,
{
    let tab_list = props.tabs_list;
    let tab_fn = props.tab_fn;
    let active_tab_signal_factory = props.active_tab_signal_factory;
    let on_tab_change = props.on_tab_change;
    html!("div", {
        .class("dmat-tabs")
        .apply(mixin)
        .children_signal_vec(tab_list.map(move |v| {
            tab(tab_fn(v), v, active_tab_signal_factory(v), on_tab_change.clone())
        }))
    })
}

fn tab<
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    TIsActiveSignal: Signal<Item = bool> + 'static,
>(
    content_node: Dom,
    tab_id: TabId,
    is_active: TIsActiveSignal,
    select_cb: Option<TabChangeCallbackFnMut<TabId>>,
) -> Dom {
    html!("button", {
        .children(&mut [
            content_node,
            html!("span", {
                .class("dmat-tab-indicator")
            })
        ])
        .class("tab")
        .class_signal("active", is_active)
        .event(clone!(select_cb => move |_: events::Click| {
            if let Some(cb) = &select_cb {
                cb.borrow_mut()(tab_id)
            }
        }))
    })
}
