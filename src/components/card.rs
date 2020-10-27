use dominator::{Dom, html};

struct CardData {
    pub header: Option<Dom>,
    pub body: Dom,
    pub footer: Option<Dom>,
}

pub struct Card {
    data: CardData
}

impl Card {
    #[inline]
    pub fn new(body: Dom) -> Self {
        Card {
            data: CardData {
                header: None,
                body,
                footer: None,
            }
        }
    }

    #[inline]
    pub fn header(mut self, header: Dom) -> Self {
        self.data.header = Some(header);
        self
    }

    #[inline]
    pub fn footer(mut self, footer: Dom) -> Self {
        self.data.footer = Some(footer);
        self
    }

    pub fn render(self) -> Dom {
        self.data.render()
    }
}

impl CardData {
    #[inline]
    fn render(self) -> Dom {
        let children = vec![
            match self.header {
                Some(header) => Some(html!("div", {
                    .class("header")
                    .class("card-section")
                    .child(header)
                })),
                _ => None
            },
            Some(html!("div", {
                .class("body")
                .class("card-section")
                .child(self.body)
            })),
            match self.footer {
                Some(footer) => Some(html!("div", {
                    .class("footer")
                    .class("card-section")
                    .child(footer)
                })),
                _ => None
            }
        ];

        html!("div", {
            .class("dmat-card")
            .children(children.into_iter().filter_map(|v| v))
        })
    }
}

