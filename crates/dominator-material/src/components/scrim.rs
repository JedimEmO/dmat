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

#[cfg(test)]
mod test {
    use dominator::html;
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;

    use dominator_testing::{
        as_html_element, async_yield, get_elements_by_class_name, has_class_name, mount_test_dom,
    };

    use crate::components::ScrimProps;
    use crate::utils::mixin::with_id;
    use crate::utils::mixin::with_stream_flipflop;

    #[wasm_bindgen_test]
    async fn test_scrim_click_toggle() {
        let visible = Mutable::new(true);

        let (scrim_dom, scrim_out) = scrim!(
            ScrimProps {
                content: html!("div"),
                hide_signal: visible.signal_ref(|v| !v)
            },
            with_id("test-scrim")
        );

        let flipflop_mixin = with_stream_flipflop(scrim_out.click_stream, visible.clone());

        let outter = html!("div", {
            .child(scrim_dom)
            .apply(flipflop_mixin)
        });

        mount_test_dom(outter);

        // Click the overlay element
        get_elements_by_class_name("scrim-overlay")
            .into_iter()
            .for_each(|e| as_html_element(&e).click());

        async_yield().await;

        assert_eq!(visible.get(), false);

        let overlays = get_elements_by_class_name("scrim-overlay");

        // Ensure that the overlay is now -hidden
        overlays
            .iter()
            .for_each(|ele| assert!(has_class_name(as_html_element(ele), "-hidden")));

        async_yield().await;

        visible.set(true);

        async_yield().await;

        let overlays = get_elements_by_class_name("scrim-overlay");

        // The overlay should now be visible, i.e not have the -hidden class
        overlays
            .iter()
            .for_each(|ele| assert!(!has_class_name(as_html_element(ele), "-hidden")))
    }
}
