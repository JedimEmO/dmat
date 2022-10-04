use dominator::Dom;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, HtmlElement};

pub fn mount_test_dom(dom: Dom) {
    dominator::append_dom(
        &web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap(),
        dom,
    );
}

///
pub fn test_dyn_element_by_id<T, F>(id: &str, tester: F)
where
    F: FnOnce(&T),
    T: JsCast,
{
    let cmp = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("Element #{} not found", id));

    tester(
        cmp.dyn_ref::<T>().unwrap_or_else(|| panic!("Element #{} is not castable to the requested element type",
                id)),
    );
}

pub fn get_elements_by_class_name(class_name: &str) -> Vec<Element> {
    let mut out = vec![];

    let elements = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_class_name(class_name);

    for n in 0..elements.length() {
        let cmp = elements.item(n).unwrap();

        out.push(cmp)
    }

    out
}

pub fn as_html_element<T: JsCast>(ele: &T) -> &HtmlElement {
    ele.dyn_ref::<HtmlElement>()
        .expect("The provided element is not castable to HtmlElement")
}

pub fn has_class_name(ele: &Element, class_name: &str) -> bool {
    ele.class_name().contains(class_name)
}

pub async fn async_yield() {
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null()))
        .await
        .unwrap();
}
