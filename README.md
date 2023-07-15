# dmat

These crates aim to provide basic functional reactive components to be used for writing single page UI applications using the dominator framework. It loosely follows the material design guidelines, but are not bound by them

The example app is hosted on github pages here:
<a href="https://jedimemo.github.io/dmat/#/component/appbar" target="_blank">https://jedimemo.github.io/dmat/examples </a>

Docs are hosted here:
<a href="https://jedimemo.github.io/dmat/#/component/appbar" target="_blank">https://jedimemo.github.io/dmat/doc/dmat_components/index.html </a>
## dmat-components


All components have a corresponding macro rule, which will insert the identity mixin for us by default to avoid pointless ```|d| d``` parameters everywhere:

```rust
// Text with no mixin
text!("Hi there!");

// Text with an ID property mixin:
text!("Hi, I have an id!", with_id("some-text-element-id"));
```


# dmat-components-style

The `dmat-components-style` crate provides a themeable default style implementation, which can be generated compile time in your `build.rs` file:

```rust
use dmat_components_style::render_dmat_scss;
use dmat_components_style::theme::{Colors, DmatTheme};
use std::fs;

fn main() {
    let theme = DmatTheme {
        colors: Colors {
            ..Default::default()
        },
    };

    let scss_file_content = render_dmat_scss("example-app", theme);

    fs::write("style/dmat.generated.scss", scss_file_content.as_str()).unwrap();
}
```

## Examples

To run the showcase application locally, go to the `examples/component_showcase` directory, and do the following:

Requires 
* rust 1.70.0+ 
* `wasm32-unknown-unknown` toolchain (`rustup target add wasm32-unknown-unknown`)
* `node` and `npm` installed

```
npm install
npm start
``` 

## Tests

To run tests in chrome, you need
* `chromedriver` installed
* `wasm-bindgen-cli` installed (`cargo install wasm-bindgen-cli`). This must match the version of `wasm-bindgen` in the `Cargo.toml` file.

now do `CHROMEDRIVER=/path/to/chromedriver cargo test --target wasm32-unknown-unknown`