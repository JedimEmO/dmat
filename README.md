# dmat

These crates aim to provide basic functional reactive components to be used for writing single page UI applications using the dominator framework. It loosely follows the material design guidelines, but are not bound by them

The example app is hosted on github pages here:
<a href="https://jedimemo.github.io/dmat/#/component/appbar" target="_blank">https://jedimemo.github.io/dmat/ </a>

## dmat-components
Every component provided in this cate is implemented as a function, and there are two types of component functions.

The simple type takes properties and a mixin function as arguments, and return a Dom element:

```rust
fn my_cmp<F>(props: MyCmpProps, mixin: F) -> Dom
    where F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> { 
    /* ... */
}
```

A more complex type will return a ```(Dom, MyCmpOut)``` tuple, where `MyCmpOut` has outputs that will be used to trigger actions in your application.

For instance, here is the definition of the scrim components output, which returns a stream of unit values representing clicks on the scrim overlay:

```rust
pub struct ScrimOut {
    pub click_stream: Receiver<()>,
}
pub fn scrim<THideSig, F>(props: ScrimProps<THideSig>, mixin: F) -> (Dom, ScrimOut)
    where
        THideSig: Signal<Item = bool> + 'static,
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{ /* ... */ }
```

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
* rust 1.47.0+ 
* `wasm32-unknown-unknown` toolchain (`rustup target add wasm32-unknown-unknown`)
* trunk (https://trunkrs.dev/) (`cargo install trunk`)

```
trunk serve --release
``` 

Go to http://localhost:8080 to see the showcase.
