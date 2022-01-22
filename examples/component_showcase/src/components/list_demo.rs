use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use futures_signals::signal_vec::{MutableVec, SignalVecExt};

use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{ButtonContent, ButtonProps, ButtonType, CardProps};

pub fn list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    card!(
        CardProps::new().body(static_list!(vec![
            button!(ButtonProps {
                content: Some(ButtonContent::Dom(text!("Add new entry"))),
                click_handler: Some(Rc::new(clone!(entries => move |_| {
                    entries.lock_mut().push_cloned("Hello!".into());
                }))),
                button_type: ButtonType::Contained,
                style: Default::default(),
                disabled_signal: None
            }
            .disabled_signal(entries.signal_vec_cloned().len().map(|v| v >= 5))),
            list!(entries
                .signal_vec_cloned()
                .map(|entry| html!("span", { .text(entry.as_str())}))),
        ])),
        |v| v.class("demo-card")
    )
}
