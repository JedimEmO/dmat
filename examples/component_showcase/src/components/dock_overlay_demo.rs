use dmat_components::components::layouts::*;
use dmat_components::components::*;
use dominator::{clone, events, html, Dom};
use futures_signals::signal::Mutable;
use lipsum::lipsum;

pub fn dock_overlay_demo() -> Dom {
    let show_overlay = Mutable::new(true);

    let underlying_view = button!({
            .label("Show overlay")
            .click_handler(clone!(show_overlay => move |_: events::Click| show_overlay.set(true)))
    });

    dock_overlay!({
        .underlying_view(Some(underlying_view))
        .overlay_views(vec![
            (DockPoint::MiddleLeft, card!({.child(text!("Middle Left Dialog!"))})),
            (DockPoint::MiddleCenter, middle_center_dialog(show_overlay.clone())),
            (DockPoint::MiddleRight, html!("span", {.text("Middle Right Dialog!")})),
            (DockPoint::TopLeft, html!("span", {.text("Top Left Dialog!")})),
            (DockPoint::TopCenter, html!("span", {.text("Top Center Dialog!")})),
            (DockPoint::TopRight, html!("span", {.text("Top Right Dialog!")})),
            (DockPoint::BottomLeft, html!("span", {.text("Bottom Left Dialog!")})),
            (DockPoint::BottomCenter, html!("span", {.text("Bottom Center Dialog!")})),
            (DockPoint::BottomRight, html!("span", {.text("Bottom Right Dialog!")})),
        ])
        .show_scrim(true)
        .show_overlay_signal(show_overlay.signal())
    })
}

fn middle_center_dialog(show_overlay: Mutable<bool>) -> Dom {
    card!({
        .child(
            content_block!(ContentBlockProps {
                title_section: Some(title!({
                    .header_text("Card with content block".to_string())
                    .sub_header_text(Some("All sections".to_string()))
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
    })
}
