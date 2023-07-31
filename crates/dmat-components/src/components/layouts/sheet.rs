use crate::components::scrim::*;
use dominator::{ events, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use wasm_bindgen::UnwrapThrowExt;

/// Modal sheets are used to display supplementary content anchored to the sides of the screen
/// They can render a scrim over the underlying UI, and must me dismissed to return to the application
///
/// <https://material.io/components/sheets-side>
/// <https://material.io/components/sheets-bottom>
#[component(render_fn = render_sheet)]
pub struct Sheet<TOnScrimClick: Fn(events::Click) -> () = fn(events::Click) -> ()> {
    #[signal]
    #[default(SheetSide::Left)]
    side: SheetSide,
    #[signal]
    #[default(None)]
    underlying_view: Option<Dom>,
    #[signal]
    #[default(None)]
    sheet_content: Option<Dom>,
    #[signal]
    #[default(true)]
    expanded: bool,
    #[signal]
    #[default(false)]
    show_scrim: bool,
    #[default(|_| {})]
    on_scrim_click: TOnScrimClick,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SheetSide {
    Left,
    Right,
    Bottom,
}

/// Returns the tuple (0: view, 1: Scrim click stream of ())
pub fn render_sheet(props: impl SheetPropsTrait + 'static) -> Dom {
    let SheetProps {
        side,
        underlying_view,
        sheet_content,
        expanded,
        show_scrim,
        on_scrim_click,
        apply,
    } = props.take();

    let side_bc = side.broadcast();
    let expanded_bc = expanded.broadcast();

    let do_show_scrim = map_ref! {
        let show_scrim_v = show_scrim,
        let expanded_v = expanded_bc.signal_ref(|v| *v) => {
            *show_scrim_v && *expanded_v
        }
    };

    let wrapped = scrim(
        ScrimProps::new()
            .hide_signal(do_show_scrim.map(|v| !v))
            .on_click(on_scrim_click)
            .content_signal(underlying_view),
    );

    html!("div", {
        .class("dmat-sheet")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap_throw()))
        .children(&mut [
            html!("div", {
                .class("dmat-sheet-content")
                .class_signal("dmat-sheet-left", side_bc.signal().map(|v| v == SheetSide::Left))
                .class_signal("dmat-sheet-right", side_bc.signal().map(|v| v == SheetSide::Right))
                .class_signal("dmat-sheet-bottom", side_bc.signal().map(|v| v == SheetSide::Bottom))
                .class_signal("-expanded", expanded_bc.signal_ref(|v| *v))
                .child_signal(sheet_content)
            }),
            html!("div", {
                .class("dmat-sheet-main-view")
                .child(wrapped)
            })
        ])
    })
}
