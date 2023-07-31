use dominator::{events, html, Dom};

/// Overlays a semi-opaque toggleable scrim over a component
#[component(render_fn = scrim)]
struct Scrim<TOnClick: Fn(events::Click) -> () = fn(events::Click) -> ()> {
    /// The Dom that will be overlaid by the scrim when it is visible
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    /// bool signal which toggles the visibility of the shaded overlay
    #[signal]
    #[default(false)]
    hide: bool,

    #[default(| _ | {})]
    on_click: TOnClick,
}

/// Overlays a semi-opaque toggleable scrim over a component
pub fn scrim(props: impl ScrimPropsTrait + 'static) -> Dom {
    let ScrimProps {
        content,
        hide,
        apply,
        on_click,
    } = props.take();

    html!("div", {
        .class("dmat-scrim")
        .apply_if(apply.is_some(), |dom| {
            apply.unwrap()(dom)
        })
        .child_signal(content)
        .child(html!("div", {
            .class("scrim-overlay")
            .class_signal("-hidden", hide)
            .event(move |e: events::Click |{
                on_click(e);
            })
        }))
    })
}

#[cfg(test)]
mod test {
    use dominator::{clone, html};
    use futures_signals::signal::Mutable;
    use wasm_bindgen_test::*;

    use dominator_testing::{
        as_html_element, async_yield, get_elements_by_class_name, has_class_name, mount_test_dom,
    };

    use crate::components::scrim::*;
    use crate::utils::mixin::id_attribute_mixin;

    #[wasm_bindgen_test]
    async fn test_scrim_click_toggle() {
        let visible = Mutable::new(true);

        let scrim_dom = scrim!({
            .content(Some(html!("div")))
            .hide_signal(visible.signal_ref(|v| !v))
            .on_click(clone!(visible => move |_| {
                visible.set(!visible.get());
            }))
            .apply(id_attribute_mixin("test-scrim"))
        });

        let outter = html!("div", {
            .child(scrim_dom)
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
