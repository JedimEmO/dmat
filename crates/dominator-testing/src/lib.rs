use dominator::Dom;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

pub fn mount_test_dom(dom: Dom) -> () {
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

pub fn test_dyn_element_by_id<T, F>(id: &str, tester: F)
where
    F: FnOnce(&T) -> (),
    T: JsCast,
{
    let cmp = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap();

    tester(cmp.dyn_ref::<T>().unwrap());
}

pub async fn async_yield() {
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null()))
        .await
        .unwrap();
}
