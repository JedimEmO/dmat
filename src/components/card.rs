use dominator::{Dom, html};

pub struct Card {
    pub(crate) _header: Option<Box<dyn Fn() -> Dom>>,
    pub(crate) body: Box<dyn Fn() -> Dom>,
    pub(crate) _footer: Option<Box<dyn Fn() -> Dom>>,
}

impl Card {
    pub fn build<F: 'static>(body: F) -> Card
        where F: Fn() -> Dom {
        Card {
            _header: None,
            body: Box::new(body),
            _footer: None,
        }
    }

    pub fn dom(self) -> Dom {
        card(self)
    }
}

fn card(panel: Card) -> Dom {
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
