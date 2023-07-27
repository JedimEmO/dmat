use dominator::{clone, events, html, Dom};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;

/// Navigation tabs
///
/// # Example
/// ```rust,no_run
/// use dmat_components::components::tabs::*;
/// use dominator::{clone, html, Dom};
/// use futures_signals::signal::Mutable;
///
/// fn tabs_example() -> Dom {
///     let active_tab_index = Mutable::new(0);
///     tabs!({
///      .active_tab_signal(active_tab_index.signal())
///      .tab_click_handler(clone!(active_tab_signal => move |idx| active_tab_index.set(idx)))
///      .tabs(vec![
///          html!("div", {
///              .text("About")
///          }),
///          html!("div", {
///              .text("Components")
///          }),
///          html!("div", {
///              .text("Visualization Components")
///          }),
///      ]))
/// }
///```
#[component(render_fn = tabs)]
struct Tabs<TClickHandler: Fn(usize) = fn(usize) -> ()> {
    #[signal_vec]
    #[default(vec ! [])]
    tabs: Dom,

    #[signal]
    #[default(None)]
    active_tab: Option<usize>,

    #[default(|_| {})]
    tab_click_handler: TClickHandler,
}

#[inline]
pub fn tabs(props: impl TabsPropsTrait + 'static) -> Dom {
    let TabsProps {
        tabs,
        active_tab,
        tab_click_handler,
        apply,
    } = props.take();

    let active_tab_bc = active_tab.broadcast();
    let handler = Rc::new(tab_click_handler);

    html!("div", {
        .class("dmat-tabs")
        .apply_if(apply.is_some(), |dom| apply.unwrap()(dom))
        .children_signal_vec(tabs.enumerate().map(move |(idx_tab,tab_content)| {
            html!("button", {
                .children(&mut [
                    tab_content,
                    html!("span", {
                        .class("dmat-tab-indicator")
                    })
                ])
                .class("tab")
                .class_signal("active", active_tab_bc.signal_ref(clone!(idx_tab => move |idx_active| idx_active == &idx_tab.get())))
                .event(clone!(handler => move |_: events::Click| {
                    handler(idx_tab.get().unwrap());
                }))
            })
        }))
    })
}
