use dominator::{clone, html, Dom};

use futures_signals::signal_vec::{always, MutableVec, SignalVecExt};

use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{
    button, card, list, text, ButtonProps, ButtonType, CardProps,
};

pub fn list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    Dom::with_state(entries, |state| {
        card(CardProps::new().body(list(always(vec![
                    button(ButtonProps {
                        content_signal: Some(text("Add new entry").into()),
                        click_handler: Some(Rc::new(clone!(state => move |_| {
                            state.lock_mut().push_cloned("Hello!".into());
                        }))),
                        button_type: ButtonType::Contained,
                    }),
                    list(
                        state
                            .signal_vec_cloned()
                            .map(|entry| html!("span", { .text(entry.as_str())})),
                    ),
                ]))))
        .apply(|v| v.class("demo-card"))
        .into_dom()
    })
}
