use dominator::{html, Dom, DomBuilder};
use futures::channel::mpsc::Receiver;
use futures_signals::signal::{always, Signal, SignalExt};
use web_sys::HtmlElement;

use crate::components::ScrimProps;
use crate::scrim;

#[macro_export]
macro_rules! dock_overlay {
    ($props: expr) => {{
        $crate::components::layouts::dock_overlay::dock_overlay($props, |d| d)
    }};

    ($props: expr, $mixin: expr) => {{
        $crate::components::layouts::dock_overlay::dock_overlay($props, $mixin)
    }};
}

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

pub struct DockOverlayProps<
    TOverlayViewSignal: Signal<Item = Option<Dom>>,
    TShowOverlaySignal: Signal<Item = bool>,
> {
    pub inner_view: Dom,
    pub dock_point: DockPoint,
    pub overlay_view_signal: TOverlayViewSignal,
    pub show_scrim: bool,
    pub show_overlay_signal: TShowOverlaySignal,
}

pub struct DockOverlayOut {
    pub scrim_click_stream: Option<Receiver<()>>,
}

/// Renders an overlay with 9 of dock points for UI elements
/// Elements in the dock positions will hover over the inner view.
/// This can be used to represent dialogs, FABs, status messages etc.
///
/// # Examples
/// ```no_run
/// use dmat_components::components::layouts::dock_overlay::{DockOverlayProps, DockPoint};
/// use futures_signals::signal::always;
/// use futures_signals::signal::Mutable;
/// let show_overlay = Mutable::new(true);
///
/// dmat_components::dock_overlay!(DockOverlayProps {
///         inner_view: dmat_components::container!(|d| d.child(dmat_components::text!("This view will have an overlay"))),
///         dock_point: DockPoint::MiddleCenter,
///         show_overlay_signal: show_overlay.signal(),
///         show_scrim: true,
///         overlay_view_signal: always(Some(dmat_components::card!({
///             .child(dmat_components::text!("A dialog!"))
///         }))),
///     });
/// ```
pub fn dock_overlay<TOverlayViewSignal, TShowOverlaySignal, F>(
    props: DockOverlayProps<TOverlayViewSignal, TShowOverlaySignal>,
    mixin: F,
) -> (Dom, DockOverlayOut)
where
    TOverlayViewSignal: Signal<Item = Option<Dom>> + 'static,
    TShowOverlaySignal: Signal<Item = bool> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let overlay_view_signal = props.overlay_view_signal;
    let dock_point = props.dock_point;
    let show_overlay_signal = props.show_overlay_signal;

    let mut out = DockOverlayOut {
        scrim_click_stream: None,
    };

    let inner_dom = if props.show_scrim {
        let (inner_dom, _inner_scrim_out) = scrim!(ScrimProps {
            hide_signal: always(false),
            content: props.inner_view
        });

        out.scrim_click_stream = Some(_inner_scrim_out.click_stream);
        inner_dom
    } else {
        props.inner_view
    };

    (
        html!("div", {
            .class("dmat-dock-overlay")
            .class_signal("-hidden", show_overlay_signal.map(|v| !v))
            .apply(mixin)
            .children(&mut [
                inner_dom,
                html!("div", {
                    .class("dmat-overlay-view")
                    .class(dock_point_to_css_class(dock_point))
                    .child_signal(overlay_view_signal)
                })
            ])
        }),
        out,
    )
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
