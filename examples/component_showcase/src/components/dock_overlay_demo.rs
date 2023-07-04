use dmat_components::components::layouts::ContentBlockProps;
use dmat_components::components::layouts::{DockOverlayOut, DockOverlayProps, DockPoint};
use dmat_components::components::TitleProps;
use dmat_components::utils::signals::stream_flipflop::stream_to_flipflop_mixin;
use dominator::{clone, events, html, Dom};
use futures_signals::signal::{always, Mutable, ReadOnlyMutable};
use lipsum::lipsum;

pub fn dock_overlay_demo() -> Dom {
    let show_overlay = Mutable::new(true);

    let (innermost_overlay, innermost_overlay_out) = middle_center_dialog(show_overlay.clone());
    let scrim_click_stream = innermost_overlay_out
        .scrim_click_stream
        .expect("innermost demo dock overlay should have a scrim");

    let flipflop_mixin = stream_to_flipflop_mixin(scrim_click_stream, &show_overlay);

    let middle_left = dock_overlay!(DockOverlayProps {
        inner_view: innermost_overlay,
        dock_point: DockPoint::MiddleLeft,
        show_overlay_signal: show_overlay.signal(),
        show_scrim: false,
        overlay_view_signal: always(Some(
            card!({.child(text!("Middle Left Dialog!", flipflop_mixin))})
        )),
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
        overlay_view_signal: always(Some(card!({.child(text!(content))}))),
    })
    .0
}

fn middle_center_dialog(show_overlay: Mutable<bool>) -> (Dom, DockOverlayOut) {
    dock_overlay!(DockOverlayProps {
        inner_view: container!(|d| d.child(button!({
            .label("Show overlay")
            .click_handler(clone!(show_overlay => move |_: events::Click| show_overlay.set(true)))
        }))),
        dock_point: DockPoint::MiddleCenter,
        show_overlay_signal: show_overlay.signal(),
        show_scrim: true,
        overlay_view_signal: always(Some(card!({
            .child(         content_block!(ContentBlockProps {
                title_section: Some(title!(TitleProps {
                    header_text_signal: always("Card with content block".to_string()),
                    sub_header_text_signal: always(Some("All sections".to_string())),
                })),
                media_section: Some(html!("img", {
                    .attr("src", "images/shapes.svg")
                    .attr("width", "100%")
                    .attr("height", "100%")
                    .attr("alt", "shapes!")
                })),
                supporting_section: Some(text!(lipsum(30))),
                footer_section: Some(button!({
                    .label("Hide overlay")
                    .click_handler(clone!(show_overlay => move |_: events::Click| show_overlay.set(false)))
                })),
            }))
            .apply(|d| d.style("width", "300px"))
        }))),
    })
}
