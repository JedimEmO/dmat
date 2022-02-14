use dominator::{html, Dom};
use dominator_material::components::layouts::{ModalSheetProps, SheetProps, SheetSide};
use dominator_material::components::ButtonProps;
use dominator_material::utils::signals::stream_flipflop::stream_to_flipflop_mixin;
use futures_signals::signal::always;
use futures_signals::signal::Mutable;

pub fn sheet_demo() -> Dom {
    bottom_sheet()
}

fn bottom_sheet() -> Dom {
    let show_bottom = Mutable::new(true);
    let show = show_bottom.clone();
    let expanded_signal = show_bottom.signal_cloned();

    let (sheet_dom, modal_sheet_out) = modal_sheet!(ModalSheetProps {
        sheet_props: SheetProps {
            sheet_content: text!("Bottom sheet"),
            wrapped_view: html!("div", {
                .child(left_sheet(
                    show_bottom
                ))
            }),
            side: SheetSide::Bottom,
            expanded_signal
        }
    });

    let flipflop_mixin = stream_to_flipflop_mixin(modal_sheet_out.toggle_stream, &show);

    html!("div", {
        .apply(flipflop_mixin)
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
            button!(
                ButtonProps::new(move |_| show_left.set(!show_left.get()), always(false))
                    .content(text!("Toggle left sheet"))
            ),
            button!(
                ButtonProps::new(move |_| show_bottom.set(!show_bottom.get()), always(false))
                    .content(text!("Toggle bottom sheet"))
            ),
            button!(
                ButtonProps::new(move |_| show_right.set(!show_right.get()), always(false))
                    .content(text!("Toggle right sheet"))
            )
        ])),
        side: SheetSide::Right,
        expanded_signal
    })
}
