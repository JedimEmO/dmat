use dominator::DomBuilder;
use web_sys::Element;

#[inline]
pub fn new_html(node: &str) -> DomBuilder<Element> {
    DomBuilder::new_html(node)
}

#[cfg(test)]
mod test {
    use crate::elements::elements::new_html;
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
