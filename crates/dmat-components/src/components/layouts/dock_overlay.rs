use crate::futures_signals::signal_vec::SignalVecExt;
use dominator::{events, html, Dom};
use futures_signals::signal::SignalExt;

use crate::components::scrim::*;

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

/// Renders an overlay with 9 of dock points for UI elements
/// Elements in the dock positions will hover over the inner view.
/// This can be used to represent dialogs, FABs, status messages etc.
///
/// # Examples
/// ```no_run
/// use dmat_components::components::layouts::dock_overlay::*;
/// use futures_signals::signal::always;
/// use futures_signals::signal::Mutable;
///
/// let show_overlay = Mutable::new(true);
///
/// dock_overlay!({
///  .underlying_view(Some(underlying_view))
///  .overlay_views(vec![
///      (DockPoint::MiddleLeft, card!({.child(text!("Middle Left Dialog!"))})),
///      (DockPoint::MiddleCenter, middle_center_dialog(show_overlay.clone())),
///      (DockPoint::MiddleRight, html!("span", {.text("Middle Right Dialog!")})),
///      (DockPoint::TopLeft, html!("span", {.text("Top Left Dialog!")})),
///      (DockPoint::TopCenter, html!("span", {.text("Top Center Dialog!")})),
///      (DockPoint::TopRight, html!("span", {.text("Top Right Dialog!")})),
///      (DockPoint::BottomLeft, html!("span", {.text("Bottom Left Dialog!")})),
///      (DockPoint::BottomCenter, html!("span", {.text("Bottom Center Dialog!")})),
///      (DockPoint::BottomRight, html!("span", {.text("Bottom Right Dialog!")})),
///  ])
///  .show_scrim(true)
///  .show_overlay_signal(show_overlay.signal())
/// })
/// ```
#[component(render_fn = dock_overlay)]
struct DockOverlay<TOnScrimClick: Fn(events::Click) = fn(events::Click) -> ()> {
    #[signal]
    #[default(None)]
    pub underlying_view: Option<Dom>,
    #[signal_vec]
    #[default(vec ! [])]
    pub overlay_views: (DockPoint, Dom),
    #[signal]
    #[default(false)]
    pub show_scrim: bool,
    #[signal]
    #[default(true)]
    pub show_overlay: bool,
    #[default(| _ | {})]
    pub on_scrim_click: TOnScrimClick,
}

pub fn dock_overlay(props: impl DockOverlayPropsTrait + 'static) -> Dom {
    let DockOverlayProps {
        underlying_view,
        overlay_views,
        show_scrim,
        show_overlay: show_overlay_signal,
        on_scrim_click,
        apply,
    } = props.take();

    let view = scrim(
        ScrimProps::new()
            .content_signal(underlying_view)
            .on_click(on_scrim_click)
            .hide_signal(show_scrim.map(|v| !v)),
    );

    let children = overlay_views.map(|(dock_point, overlay_view)| {
        html!("dom", {
            .class("dmat-overlay-view")
            .class(dock_point_to_css_class(dock_point))
            .child(overlay_view)
        })
    });

    html!("div", {
        .class("dmat-dock-overlay")
        .class_signal("-hidden", show_overlay_signal.map(|v| !v))
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .child(view)
        .children_signal_vec(children)
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
