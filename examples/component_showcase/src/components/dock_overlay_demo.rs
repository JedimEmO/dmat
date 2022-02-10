use dominator::Dom;
use dominator_material::components::CardProps;
use futures_signals::signal::always;

use dominator_material::components::layouts::{dock_overlay, DockOverlayProps, DockPoint};

pub fn dock_overlay_demo() -> Dom {
    let middle_left = generic_dialog(
        middle_center_dialog(),
        DockPoint::MiddleLeft,
        "Middle Left Dialog!",
    );
    let middle_right = generic_dialog(middle_left, DockPoint::MiddleRight, "Middle Right Dialog!");
    let top_left = generic_dialog(middle_right, DockPoint::TopLeft, "Top Left Dialog!");
    let top_center = generic_dialog(top_left, DockPoint::TopCenter, "Top Center Dialog!");
    let top_right = generic_dialog(top_center, DockPoint::TopRight, "Top Right Dialog!");
    let bottom_left = generic_dialog(top_right, DockPoint::BottomLeft, "Bottom Left Dialog!");
    let bottom_center = generic_dialog(
        bottom_left,
        DockPoint::BottomCenter,
        "Bottom Center Dialog!",
    );
    generic_dialog(
        bottom_center,
        DockPoint::BottomRight,
        "Bottom Right Dialog!",
    )
}

fn generic_dialog(inner_view: Dom, dock_point: DockPoint, content: &str) -> Dom {
    dock_overlay(DockOverlayProps {
        inner_view,
        dock_point,
        show_scrim: false,
        overlay_view_signal: always(Some(card!(CardProps {
            body_view: Some(text!(content).into()),
            footer: None,
            header_view: None
        }))),
    })
}

fn middle_center_dialog() -> Dom {
    dock_overlay(DockOverlayProps {
        inner_view: container!(|d| d.child(text!("Covered view"))),
        dock_point: DockPoint::MiddleCenter,
        show_scrim: true,
        overlay_view_signal: always(Some(card!(CardProps {
            body_view: Some(text!("A dialog!").into()),
            footer: None,
            header_view: None
        }))),
    })
}
