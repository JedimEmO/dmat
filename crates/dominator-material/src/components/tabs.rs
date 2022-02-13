use dominator::{events, html, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures_signals::signal::Signal;
use futures_signals::signal_vec::SignalVec;
use futures_signals::signal_vec::SignalVecExt;
use std::fmt::Debug;
use std::sync::Mutex;
use web_sys::HtmlElement;

pub struct TabsProps<
    TabList: SignalVec<Item = TabId>,
    TabId: Copy + std::cmp::PartialEq + Debug,
    FActiveSignalFactory,
    ActiveSignal,
    FTabRender: Fn(TabId) -> Dom,
> where
    FActiveSignalFactory: Fn(TabId) -> ActiveSignal,
    ActiveSignal: Signal<Item = bool>,
{
    /// This method transforms a TabId -> Dom
    pub tab_render_fn: FTabRender,
    /// Invoked per tab, this function should create a signal which indicates if the current tab is the active one or not
    pub is_active_tab_signal_factory: FActiveSignalFactory,
    pub tabs_list: TabList,
}

pub struct TabsOut<TabId: Copy + std::cmp::PartialEq + Debug> {
    pub tab_select_stream: Receiver<TabId>,
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

#[inline]
pub fn tabs<TabList, TabId, TActiveSignal, FActiveSignalFactory, F, FTabRender>(
    props: TabsProps<TabList, TabId, FActiveSignalFactory, TActiveSignal, FTabRender>,
    mixin: F,
) -> (Dom, TabsOut<TabId>)
where
    TabList: SignalVec<Item = TabId> + 'static,
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    FActiveSignalFactory: Fn(TabId) -> TActiveSignal + 'static,
    TActiveSignal: Signal<Item = bool> + 'static,
    FTabRender: Fn(TabId) -> Dom + 'static,
{
    let tab_list = props.tabs_list;
    let tab_fn = props.tab_render_fn;
    let active_tab_signal_factory = props.is_active_tab_signal_factory;

    let (tab_tx, tab_rx) = channel(1);

    (
        html!("div", {
            .class("dmat-tabs")
            .apply(mixin)
            .children_signal_vec(tab_list.map(move |v| {
                tab(tab_fn(v), v, active_tab_signal_factory(v), tab_tx.clone())
            }))
        }),
        TabsOut {
            tab_select_stream: tab_rx,
        },
    )
}

fn tab<
    TabId: Copy + std::cmp::PartialEq + Debug + 'static,
    TIsActiveSignal: Signal<Item = bool> + 'static,
>(
    content_node: Dom,
    tab_id: TabId,
    is_active: TIsActiveSignal,
    tab_tx: Sender<TabId>,
) -> Dom {
    let tab_tx = Mutex::new(tab_tx);

    html!("button", {
        .children(&mut [
            content_node,
            html!("span", {
                .class("dmat-tab-indicator")
            })
        ])
        .class("tab")
        .class_signal("active", is_active)
        .event(move |_: events::Click| {
            tab_tx.lock().unwrap().try_send(tab_id).or::<()>(Ok(())).unwrap();
        })
    })
}
