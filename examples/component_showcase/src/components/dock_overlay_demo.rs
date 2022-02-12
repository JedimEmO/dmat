use dominator::{clone, events, Dom};
use futures_signals::signal::{always, Mutable, ReadOnlyMutable};

use dominator_material::components::layouts::{DockOverlayOut, DockOverlayProps, DockPoint};
use dominator_material::components::{ButtonContent, ButtonProps, CardProps};
use dominator_material::utils::mixin::with_stream_flipflop;

pub fn dock_overlay_demo() -> Dom {
    let show_overlay = Mutable::new(true);

    let (innermost_overlay, innermost_overlay_out) = middle_center_dialog(show_overlay.clone());
    let scrim_click_stream = innermost_overlay_out
        .scrim_click_stream
        .expect("innermost demo dock overlay should have a scrim");

    let middle_left = dock_overlay!(DockOverlayProps {
        inner_view: innermost_overlay,
        dock_point: DockPoint::MiddleLeft,
        show_overlay_signal: show_overlay.signal(),
        show_scrim: false,
        overlay_view_signal: always(Some(card!(CardProps {
            body_view: Some(
                text!(
                    "Middle Left Dialog!",
                    with_stream_flipflop(scrim_click_stream, show_overlay.clone())
                )
                .into()
            ),
            footer: None,
            header_view: None
        }))),
    })
    .0;

    let middle_right = generic_dialog(
        middle_left,
        DockPoint::MiddleRight,
        "Middle Right Dialog!",
        &show_overlay,
    );
    let top_left = generic_dialog(
        middle_right,
        DockPoint::TopLeft,
        "Top Left Dialog!",
        &show_overlay,
    );
    let top_center = generic_dialog(
        top_left,
        DockPoint::TopCenter,
        "Top Center Dialog!",
        &show_overlay,
    );
    let top_right = generic_dialog(
        top_center,
        DockPoint::TopRight,
        "Top Right Dialog!",
        &show_overlay,
    );
    let bottom_left = generic_dialog(
        top_right,
        DockPoint::BottomLeft,
        "Bottom Left Dialog!",
        &show_overlay,
    );
    let bottom_center = generic_dialog(
        bottom_left,
        DockPoint::BottomCenter,
        "Bottom Center Dialog!",
        &show_overlay,
    );

    generic_dialog(
        bottom_center,
        DockPoint::BottomRight,
        "Bottom Right Dialog!",
        &show_overlay,
    )
}

fn generic_dialog(
    inner_view: Dom,
    dock_point: DockPoint,
    content: &str,
    show_overlay: &ReadOnlyMutable<bool>,
) -> Dom {
    dock_overlay!(DockOverlayProps {
        inner_view,
        dock_point,
        show_overlay_signal: show_overlay.signal(),
        show_scrim: false,
        overlay_view_signal: always(Some(card!(CardProps {
            body_view: Some(text!(content).into()),
            footer: None,
            header_view: None
        }))),
    })
    .0
}

fn middle_center_dialog(show_overlay: Mutable<bool>) -> (Dom, DockOverlayOut) {
    let show_button_props = ButtonProps {
        content: Some(ButtonContent::Label("Show overlay".to_string())),
        click_handler: clone!(show_overlay => move |_: events::Click| show_overlay.set(true)),
        button_type: Default::default(),
        style: Default::default(),
        disabled_signal: always(false),
    };

    let hide_button_props = ButtonProps {
        content: Some(ButtonContent::Label("Hide overlay".to_string())),
        click_handler: clone!(show_overlay => move |_: events::Click| show_overlay.set(false)),
        button_type: Default::default(),
        style: Default::default(),
        disabled_signal: always(false),
    };

    dock_overlay!(DockOverlayProps {
        inner_view: container!(|d| d.child(button!(show_button_props))),
        dock_point: DockPoint::MiddleCenter,
        show_overlay_signal: show_overlay.signal(),
        show_scrim: true,
        overlay_view_signal: always(Some(card!(CardProps {
            body_view: Some(
                static_list!(vec![text!("A dialog!").into(), button!(hide_button_props)]).into()
            ),
            footer: None,
            header_view: None
        }))),
    })
}