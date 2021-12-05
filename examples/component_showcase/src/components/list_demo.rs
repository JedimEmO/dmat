use dominator::{clone, html, Dom};
use futures_signals::signal_vec::{always, MutableVec, SignalVecExt};
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{button, list, ButtonProps, ButtonType, Card};

pub fn list_demo() -> Dom {
    let entries: Rc<MutableVec<String>> = Default::default();

    Dom::with_state(entries, |state| {
        Card::new()
            .apply(|v| v.class("demo-card"))
            .body(list(always(vec![
                button(ButtonProps {
                    content: Box::new(|| html!("span", { .text("Add Entry")})),
                    click_handler: Some(Rc::new(clone!(state => move |_| {
                        state.lock_mut().push_cloned("Hello!".into());
                    }))),
                    button_type: ButtonType::Contained,
                }),
                list(
                    state
                        .signal_vec_cloned()
                        .map(|entry| html!("span", { .text(format!("{}", entry).as_str())})),
                ),
            ])))
            .render()
    })
}
