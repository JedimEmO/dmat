use crate::components::ScrimProps;
use crate::scrim;
use dominator::{clone, html, Dom, DomBuilder};
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use web_sys::HtmlElement;

#[macro_export]
macro_rules! sheet {
    ($a: expr) => {{
        $crate::components::layouts::sheet($a, |d| d)
    }};

    ($a: expr, $mixin: expr) => {{
        $crate::components::layouts::sheet($a, $mixin)
    }};
}

#[macro_export]
macro_rules! modal_sheet {
    ($a: expr) => {{
        $crate::components::layouts::modal_sheet($a, |d| d)
    }};

    ($a: expr, $mixin: expr) => {{
        $crate::components::layouts::modal_sheet($a, $mixin)
    }};
}

pub enum SheetSide {
    Left,
    Right,
    Bottom,
}

pub struct SheetProps<TExpandedSignal: Signal<Item = bool> + Unpin + 'static> {
    pub sheet_content: Dom,
    pub wrapped_view: Dom,
    pub side: SheetSide,
    pub expanded_signal: TExpandedSignal,
}

pub struct ModalSheetProps<TExpandedSignal: Signal<Item = bool> + Unpin + 'static> {
    pub sheet_props: SheetProps<TExpandedSignal>,
}

pub struct ModalSheetOut {
    pub toggle_stream: Receiver<()>,
}

/// Sheets are used to display supplementary content anchored to the sides of the screen
///
/// <https://material.io/components/sheets-side>
/// <https://material.io/components/sheets-bottom>
pub fn sheet<THideSignal, F>(props: SheetProps<THideSignal>, mixin: F) -> Dom
where
    THideSignal: Signal<Item = bool> + Unpin + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    render_sheet(
        props.sheet_content,
        props.wrapped_view,
        props.side,
        props.expanded_signal,
        mixin,
        false,
    )
    .0
}

/// Modal sheets are used to display supplementary content anchored to the sides of the screen
/// They render a scrim over the underlying UI, and must me dismissed to return to the application
///
/// <https://material.io/components/sheets-side>
/// <https://material.io/components/sheets-bottom>
pub fn modal_sheet<TExpandedSignal, F>(
    props: ModalSheetProps<TExpandedSignal>,
    mixin: F,
) -> (Dom, ModalSheetOut)
where
    TExpandedSignal: Signal<Item = bool> + Unpin + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let (dom, strm) = render_sheet(
        props.sheet_props.sheet_content,
        props.sheet_props.wrapped_view,
        props.sheet_props.side,
        props.sheet_props.expanded_signal,
        mixin,
        true,
    );

    (
        dom,
        ModalSheetOut {
            toggle_stream: strm,
        },
    )
}

/// Returns the tuple (0: view, 1: Scrim click stream of ())
fn render_sheet<TExpandedSignal, F>(
    content: Dom,
    wrapped: Dom,
    side: SheetSide,
    expand: TExpandedSignal,
    mixin: F,
    do_show_scrim: bool,
) -> (Dom, Receiver<()>)
where
    TExpandedSignal: Signal<Item = bool> + Unpin + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let sheet_content_class = sheet_side_to_content_class(side);
    let show_scrim = Mutable::new(false);

    let (wrapped, scrim_out) = scrim!(ScrimProps {
        hide_signal: show_scrim.signal_ref(move |v| !do_show_scrim || !v),
        content: wrapped
    });

    (
        html!("div", {
            .class("dmat-sheet")
            .apply(mixin)
            .future(clone!(show_scrim => async move {
                let mut expand_stream = expand.to_stream();

                while let Some(v) = expand_stream.next().await {
                    show_scrim.set(v)
                }
            }))
            .children(&mut [
                html!("div", {
                    .class("dmat-sheet-content")
                    .class(sheet_content_class)
                    .class_signal("-expanded", show_scrim.signal_cloned())
                    .child(content)
                }),
                html!("div", {
                    .class("dmat-sheet-main-view")
                    .child(wrapped)
                })
            ])
        }),
        scrim_out.click_stream,
    )
}

fn sheet_side_to_content_class(side: SheetSide) -> &'static str {
    match side {
        SheetSide::Left => "dmat-sheet-left",
        SheetSide::Right => "dmat-sheet-right",
        SheetSide::Bottom => "dmat-sheet-bottom",
    }
}
