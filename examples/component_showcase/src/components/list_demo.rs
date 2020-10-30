use dominator::{clone, html, Dom};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{Button, List};

pub struct ListDemo {
    entries: MutableVec<String>,
}

impl ListDemo {
    pub fn new() -> Rc<ListDemo> {
        Rc::new(ListDemo {
            entries: Default::default(),
        })
    }

    pub fn render(self: Rc<Self>) -> Dom {
        Dom::with_state(self, |state| {
            List::new_static(vec![
                Button::new()
                    .text("Add entry")
                    .on_click(clone!(state => move |_| {
                        state.entries.lock_mut().push_cloned("Hello!".into());
                    }))
                    .render(),
                List::new_dynamic(
                    state
                        .entries
                        .signal_vec_cloned()
                        .map(|entry| html!("span", { .text(format!("{}", entry).as_str())})),
                ),
            ])
        })
    }
}
