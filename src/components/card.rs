use dominator::{clone, Dom, html};

pub struct Card {
    pub(crate) header: Option<Box<dyn Fn() -> Dom>>,
    pub(crate) body: Box<dyn Fn() -> Dom>,
    pub(crate) footer: Option<Box<dyn Fn() -> Dom>>,
}

impl Card {
    pub fn build<F: 'static>(body: F) -> Card
        where F: Fn() -> Dom {
        Card {
            header: None,
            body: Box::new(body),
            footer: None,
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
