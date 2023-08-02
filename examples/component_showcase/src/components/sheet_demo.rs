use dmat_components::components::button::*;
use dmat_components::components::layouts::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;

pub fn sheet_demo() -> Dom {
    bottom_sheet()
}

fn bottom_sheet() -> Dom {
    let show_bottom = Mutable::new(true);
    let expanded_signal = show_bottom.signal_cloned();

    let sheet_dom = sheet!({
        .sheet_content(Some(html!("span", {.text("Bottom sheet") })))
        .underlying_view(Some(html!("div", {
            .child(left_sheet(
                show_bottom.clone()
            ))
        })))
        .side(SheetSide::Bottom)
        .expanded_signal(expanded_signal)
        .show_scrim(true)
        .on_scrim_click(clone!(show_bottom => move |_| show_bottom.set(false)))
    });

    html!("div", {
        .child(sheet_dom)
    })
}

fn left_sheet(show_bottom: Mutable<bool>) -> Dom {
    let show_left = Mutable::new(true);
    let expanded_signal = show_left.signal_cloned();

    sheet!( {
        .sheet_content(Some(container!({.apply(|d| d.text("Left hand side sheet"))})))
        .underlying_view(Some(html!("div",  {
            .child(right_sheet(
                show_bottom,
                show_left
            ))
        })))
        .side(SheetSide::Left)
        .expanded_signal(expanded_signal)
    })
}

fn right_sheet(show_bottom: Mutable<bool>, show_left: Mutable<bool>) -> Dom {
    let show_right = Mutable::new(true);
    let expanded_signal = show_right.signal_cloned();

    sheet!({
        .sheet_content(Some(container!({.apply(|d| d.text("Right hand side sheet"))})))
        .underlying_view(Some(container!({.children([
            button!({
                .click_handler(move |_| show_left.set(!show_left.get()))
                .label("Toggle left sheet")
            }),
            button!({
                .click_handler(move |_| show_bottom.set(!show_bottom.get()))
                    .content(html!("span", { .text("Toggle bottom sheet") }))
            }),
            button!({
                .click_handler(move |_| show_right.set(!show_right.get()))
                .content(html!("span", { .text("Toggle right sheet") }))
            })
        ])})))
        .side(SheetSide::Right)
        .expanded_signal(expanded_signal)
    })
}
