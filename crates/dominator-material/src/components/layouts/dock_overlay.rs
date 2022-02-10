use dominator::{html, Dom};
use futures_signals::signal::{always, Signal};

use crate::components::ScrimProps;
use crate::scrim;

pub enum DockPoint {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub struct DockOverlayProps<TOverlayViewSignal: Signal<Item = Option<Dom>>> {
    pub inner_view: Dom,
    pub dock_point: DockPoint,
    pub overlay_view_signal: TOverlayViewSignal,
    pub show_scrim: bool,
}

/// Renders an overlay with 9 of dock points for UI elements
/// Elements in the dock positions will hover over the inner view.
/// This can be used to represent dialogs, FABs, status messages etc.
pub fn dock_overlay<TOverlayViewSignal>(props: DockOverlayProps<TOverlayViewSignal>) -> Dom
where
    TOverlayViewSignal: Signal<Item = Option<Dom>> + 'static,
{
    let overlay_view_signal = props.overlay_view_signal;
    let dock_point = props.dock_point;

    let inner_dom = if props.show_scrim {
        let (inner_dom, _inner_scrim_out) = scrim!(ScrimProps {
            hide_signal: always(false),
            content: props.inner_view
        });
        inner_dom
    } else {
        props.inner_view
    };

    html!("div", {
        .class("dmat-dock-overlay")
        .children(&mut [
            inner_dom,
            html!("div", {
                .class("dmat-overlay-view")
                .class(dock_point_to_css_class(dock_point))
                .child_signal(overlay_view_signal)
            })
        ])
    })
}

fn dock_point_to_css_class(dock_point: DockPoint) -> &'static str {
    match dock_point {
        DockPoint::TopLeft => "top-left",
        DockPoint::TopCenter => "top-center",
        DockPoint::TopRight => "top-right",
        DockPoint::MiddleLeft => "middle-left",
        DockPoint::MiddleCenter => "middle-center",
        DockPoint::MiddleRight => "middle-right",
        DockPoint::BottomLeft => "bottom-left",
        DockPoint::BottomCenter => "bottom-center",
        DockPoint::BottomRight => "bottom-right",
    }
}
