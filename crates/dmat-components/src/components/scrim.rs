use dominator::{events, html, Dom};
use futures::channel::mpsc::{channel, Receiver};
use futures_signals::signal::SignalExt;

/// Overlays a semi-opaque toggleable scrim over a component
#[component(render_fn = scrim)]
struct Scrim {
    /// The Dom that will be overlaid by the scrim when it is visible
    #[signal]
    content: Dom,
    /// bool signal which toggles the visibility of the shaded overlay
    #[signal]
    #[default(false)]
    hide: bool,
}

pub struct ScrimOut {
    pub click_stream: Receiver<()>,
}

/// Overlays a semi-opaque toggleable scrim over a component
pub fn scrim(props: impl ScrimPropsTrait + 'static) -> (Dom, ScrimOut) {
    let ScrimProps {
        content,
        hide,
        apply,
    } = props.take();

    let (mut tx, rx) = channel(1);

    (
        html!("div", {
            .class("dmat-scrim")
            .apply_if(apply.is_some(), |dom| {
                apply.unwrap()(dom)
            })
            .apply_if(content.is_some(), |dom| {
                dom.child_signal(content.unwrap().map(Some))
            })
            .child(html!("div", {
                .class("scrim-overlay")
                .class_signal("-hidden", hide)
                .event(move |_: events::Click |{
                    tx.try_send(()).unwrap_or(());
                })
            }))
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

    use crate::components::scrim::*;
    use crate::utils::mixin::id_attribute_mixin;
    use crate::utils::signals::stream_flipflop::stream_to_flipflop_mixin;

    #[wasm_bindgen_test]
    async fn test_scrim_click_toggle() {
        let visible = Mutable::new(true);

        let (scrim_dom, scrim_out) = scrim!({
            .content(html!("div"))
            .hide_signal(visible.signal_ref(|v| !v))
            .apply(id_attribute_mixin("test-scrim"))
        });

        let store_flipflop_mixin = stream_to_flipflop_mixin(scrim_out.click_stream, &visible);

        let outter = html!("div", {
            .child(scrim_dom)
            .apply(store_flipflop_mixin)
        });

        mount_test_dom(outter);

        // Click the overlay element
        get_elements_by_class_name("scrim-overlay")
            .into_iter()
            .for_each(|e| as_html_element(&e).click());

        async_yield().await;

        assert!(!visible.get());

        let overlays = get_elements_by_class_name("scrim-overlay");

        assert_eq!(overlays.len(), 1);

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
