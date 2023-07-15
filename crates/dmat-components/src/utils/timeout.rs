use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub fn timeout(cb: impl Fn() + 'static, timeout_in: Duration) {
    let timeout_cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>);

    web_sys::window().map(|window| {
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_cb.as_ref().unchecked_ref(),
            timeout_in.as_millis() as i32,
        )
    });

    timeout_cb.forget()
}
