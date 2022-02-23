#[cfg(test)]
mod test {
    use std::rc::Rc;

    use dominator::events::Click;
    use dominator::{body, clone, Dom, DomBuilder};
    use futures_signals::signal::{Mutable, SignalExt};
    use futures_util::StreamExt;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::{Document, HtmlElement};

    #[inline]
    pub fn new_html<T: AsRef<str>>(node: T) -> DomBuilder<HtmlElement> {
        DomBuilder::new_html(node.as_ref())
    }

    #[wasm_bindgen_test]
    fn create_basic_element() {
        let ele: Dom = new_html("span")
            .text("Hello, world!")
            .attribute("id", "test")
            .into_dom();

        let document: Document = web_sys::window().unwrap().document().unwrap();

        dominator::append_dom(&document.body().unwrap(), ele);

        let _found = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("test")
            .unwrap();
    }

    // This is for my own sanity and trust of the closure-based state handling soundness
    #[wasm_bindgen_test]
    async fn ensure_state_drop() {
        let state = Rc::new(Mutable::new(42));
        let child_count = Mutable::new(0);

        let _document: Document = web_sys::window().unwrap().document().unwrap();

        {
            let cmp = new_html("div")
                .attribute("id", "test2")
                .child_signal(state.signal().map(clone!(state, child_count => move |v| {
                    child_count.set(child_count.get() + 1);

                    Some(
                        crate::text!(format!("{}", v).as_str(), clone!(state => move |d| {
                            d.attribute("id", "inner")
                                .event(move |_: Click| {
                                    // This is what we are checking; that each
                                    // child yielded by this signal (which will generate a new clone of state)
                                    // will be properly discarded when a new child arrives
                                    assert_eq!(Rc::strong_count(&state), 3);
                                })
                        })),
                    )
                })));

            dominator::append_dom(&body(), cmp.into_dom());
        };

        assert_eq!(Rc::strong_count(&state), 2);

        let mut child_stream = child_count.signal().to_stream();

        state.set(state.get() + 1);

        while child_stream.next().await.unwrap() == 0 {}

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("inner")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .click();

        state.set(state.get() + 1);

        while child_stream.next().await.unwrap() < 2 {}

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("inner")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .click();

        assert_eq!(Rc::strong_count(&state), 3);
    }
}
