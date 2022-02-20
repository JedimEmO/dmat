extern crate futures_signals;

pub mod components;
pub mod elements;
pub mod utils;

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test_configure;

    wasm_bindgen_test_configure!(run_in_browser);
}
