use dominator::{clone, html, Dom};

use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{
    button, card, list, static_list, text, ButtonProps, ButtonType, CardProps,
};
use dominator_material::utils::mixin::{mixin_id, no_mixin};

pub fn list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    card(
        CardProps::new().body(static_list(
            vec![
                button(
                    ButtonProps {
                        content_signal: Some(text("Add new entry", no_mixin).into()),
                        click_handler: Some(Rc::new(clone!(entries => move |_| {
                            entries.lock_mut().push_cloned("Hello!".into());
                        }))),
                        button_type: ButtonType::Contained,
                    },
                    mixin_id(),
                ),
                list(
                    entries
                        .signal_vec_cloned()
                        .map(|entry| html!("span", { .text(entry.as_str())})),
                    mixin_id(),
                ),
            ],
            mixin_id(),
        )),
        |v| v.class("demo-card"),
    )
}
