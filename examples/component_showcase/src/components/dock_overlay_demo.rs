use dominator::{html, Dom};

use dominator_material::components::layouts::{dock_overlay, DockOverlayProps};

pub fn dock_overlay_demo() -> Dom {
    dock_overlay(DockOverlayProps {
        inner_view: container!(|d| d.child(text!("Covered view"))),
    })
}
