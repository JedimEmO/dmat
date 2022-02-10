use dominator::{html, Dom};
use futures_signals::signal::always;

use crate::components::ScrimProps;
use crate::scrim;

pub struct DockOverlayProps {
    pub inner_view: Dom,
}

pub fn dock_overlay(props: DockOverlayProps) -> Dom {
    let (inner_dom, _inner_scrim_out) = scrim!(ScrimProps {
        hide_signal: always(false),
        content: props.inner_view
    });

    html!("div", {
        .class("dmat-dock-overlay")
        .children(&mut [
            inner_dom
        ])
    })
}
