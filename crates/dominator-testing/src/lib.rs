use dominator::Dom;
use wasm_bindgen::JsCast;

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
