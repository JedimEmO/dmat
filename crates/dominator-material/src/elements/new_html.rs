use dominator::traits::AsStr;
use dominator::DomBuilder;

use std::rc::Rc;
use web_sys::Element;

#[inline]
pub fn new_html<T: AsStr>(node: T) -> DomBuilder<Element> {
    DomBuilder::new_html(node.as_str())
}

#[cfg(test)]
mod test {
    use crate::elements::new_html::new_html;
    use dominator::{clone, Dom};
    use std::rc::Rc;
    use wasm_bindgen_test::*;
    use web_sys::Document;

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

    #[wasm_bindgen_test]
    fn ensure_state_drop() {
        let state = Rc::new(42);
        let document: Document = web_sys::window().unwrap().document().unwrap();

        {
            let cmp = new_html("div")
                .attribute("id", "test")
                .apply(clone!(state => move |d| {
                    assert_eq!(Rc::strong_count(&state), 2);
                    d.text(format!("{}", state).as_str())
                }))
                .after_removed(clone!(state => move |_| {
                    // Make sure the RC would be kept alive by the callback all the way to removal from the DOM
                    assert_eq!(Rc::strong_count(&state), 2)
                }));

            dominator::append_dom(&document.body().unwrap(), cmp.into_dom());

            assert_eq!(Rc::strong_count(&state), 1);
        };

        assert_eq!(Rc::strong_count(&state), 1);

        document.get_element_by_id("test").unwrap().remove();

        assert_eq!(Rc::strong_count(&state), 1);
    }
}
