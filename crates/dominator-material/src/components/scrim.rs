use dominator::{events, html, Dom, DomBuilder};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::Signal;
use web_sys::HtmlElement;

#[macro_export]
macro_rules! scrim {
    ($a: expr) => {{
        $crate::components::scrim($a, |d| d)
    }};

    ($a: expr, $mixin: expr) => {{
        $crate::components::scrim($a, $mixin)
    }};
}

/// `content` - The Dom that will be overlaid by the scrim when it is visible
/// `hide_signal` - bool signal which toggles the visibility of the shaded overlay
pub struct ScrimProps<THideSig: Signal<Item = bool> + 'static> {
    pub content: Dom,
    pub hide_signal: THideSig,
}

pub struct ScrimOut {
    pub click_stream: Receiver<()>,
}

/// Overlays a semi-opaque toggleable scrim over a component
/// # Examples
/// ```no_run
/// use dominator::html;
/// use futures_signals::signal::Mutable;
///
/// use dominator_material::scrim;
/// use dominator_material::components::scrim::ScrimProps;
/// use dominator_material::utils::mixin::with_stream_flipflop;
/// let show_scrim = Mutable::new(true);
///
/// let (scrim_dom, scrim_out) = scrim!({ ScrimProps {
///     hide_signal: show_scrim.signal_cloned(),
///     content: html!("div", { .text("I am under the scrim!") })
/// }});
///
/// // the with_stream_flipflop method will make sure we toggle the show_scrim
/// // boolean value on every click to the scrim overlay
/// let _ = html!("div", {
///     .child(scrim_dom)
///     .apply(with_stream_flipflop(scrim_out.click_stream, show_scrim))
/// });
/// ```
pub fn scrim<THideSig, F>(props: ScrimProps<THideSig>, mixin: F) -> (Dom, ScrimOut)
where
    THideSig: Signal<Item = bool> + 'static,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    let content = props.content;
    let hide_signal = props.hide_signal;

    let (mut tx, rx) = channel(1);

    (
        html!("div", {
            .class("dmat-scrim")
            .apply(mixin)
            .children(&mut [
                content,
                html!("div", {
                    .class("scrim-overlay")
                    .class_signal("-hidden", hide_signal)
                    .event(move |_: events::Click |{
                        tx.try_send(()).or::<()>(Ok(())).unwrap();
                    })
                })
            ])
        }),
        ScrimOut { click_stream: rx },
    )
}
