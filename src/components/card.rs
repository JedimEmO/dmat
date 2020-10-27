use dominator::{Dom, html};
use wasm_bindgen::__rt::std::rc::Rc;

pub struct Card {
    pub(crate) _header: Option<Box<dyn Fn() -> Dom>>,
    pub(crate) body: Box<dyn Fn() -> Dom>,
    pub(crate) _footer: Option<Box<dyn Fn() -> Dom>>,
}

impl Card {
    pub fn new<F: 'static>(body: F) -> Self
        where F: Fn() -> Dom {
        Card {
            _header: None,
            body: Box::new(body),
            _footer: None,
        }
    }

    pub fn render(self: Self) -> Dom {
        card(Rc::new(self))
    }
}

fn card(panel: Rc<Card>) -> Dom {
    Dom::with_state(panel, |panel| {
        html!("div", {
            .class("dmat-card")
            .children(&mut [
                html!("div", {
                    .class("body")
                    .child((panel.body)())
                })
            ])
        })
    })
}
