use dominator::{html, Dom};
use futures_signals::signal::Mutable;

use dominator_material::components::layouts::{ModalSheetProps, SheetProps, SheetSide};
use dominator_material::components::ButtonProps;
use dominator_material::utils::mixin::with_stream_flipflop;

pub fn sheet_demo() -> Dom {
    container!(|d| { d.child(bottom_sheet()) })
}

fn bottom_sheet() -> Dom {
    let show_bottom = Mutable::new(true);
    let show = show_bottom.clone();
    let expanded_signal = show_bottom.signal_cloned();

    let (sheet_dom, modal_sheet_out) = modal_sheet!(ModalSheetProps {
        sheet_props: SheetProps {
            sheet_content: container!(|d| d.text("Bottom sheet")),
            wrapped_view: html!("div", {
                .child(left_sheet(
                    show_bottom
                ))
            }),
            side: SheetSide::Bottom,
            expanded_signal
        }
    });

    html!("div", {
        .apply(with_stream_flipflop(modal_sheet_out.toggle_stream, show))
        .child(sheet_dom)
    })
}

fn left_sheet(show_bottom: Mutable<bool>) -> Dom {
    let show_left = Mutable::new(true);
    let expanded_signal = show_left.signal_cloned();

    sheet!(SheetProps {
        sheet_content: container!(|d| d.text("Left hand side sheet")),
        wrapped_view: html!("div",  {
            .child(right_sheet(
                show_bottom,
                show_left
            ))
        }),
        side: SheetSide::Left,
        expanded_signal
    })
}

fn right_sheet(show_bottom: Mutable<bool>, show_left: Mutable<bool>) -> Dom {
    let show_right = Mutable::new(true);
    let expanded_signal = show_right.signal_cloned();

    sheet!(SheetProps {
        sheet_content: container!(|d| d.text("Right hand side sheet")),
        wrapped_view: container!(|d| d.children(&mut [
            button!(ButtonProps::new()
                .content(text!("Toggle left sheet"))
                .on_click(move |_| show_left.set(!show_left.get()))),
            button!(ButtonProps::new()
                .content(text!("Toggle bottom sheet"))
                .on_click(move |_| show_bottom.set(!show_bottom.get()))),
            button!(ButtonProps::new()
                .content(text!("Toggle right sheet"))
                .on_click(move |_| show_right.set(!show_right.get())))
        ])),
        side: SheetSide::Right,
        expanded_signal
    })
}
