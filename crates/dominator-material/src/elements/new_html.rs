use dominator::traits::AsStr;
use dominator::DomBuilder;

use std::rc::Rc;
use web_sys::Element;

#[inline]
pub fn new_html<T: AsStr>(node: T) -> DomBuilder<Element> {
    DomBuilder::new_html(node.as_str())
}

#[inline]
pub fn new_html_with_state<T: AsStr, A: 'static>(node: T, state: Rc<A>) -> DomBuilder<Element> {
    let ret = new_html(node);

    ret.after_removed(move |_| std::mem::drop(state))
}

#[cfg(test)]
mod test {
    use crate::elements::new_html::new_html;
    use dominator::Dom;
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
}
