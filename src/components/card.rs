use dominator::{html, Dom};

struct CardData {
    pub header: Option<Dom>,
    pub body: Option<Dom>,
    pub footer: Option<Dom>,
}

pub struct Card {
    data: CardData,
}

impl Card {
    #[inline]
    pub fn new() -> Self {
        Card {
            data: CardData {
                header: None,
                body: None,
                footer: None,
            },
        }
    }

    pub fn title<A: Into<String>>(mut self, title: A, sub_title: Option<A>) -> Self {
        self.data.header = Some(html!("div", {
            .children(vec![
                Some(html!("div", { .text(title.into().as_str()) })),
                match sub_title {
                    Some(sub) => Some(html!("div", { .class("sub-title") .text(sub.into().as_str()) })),
                    _ => None
                }
            ].into_iter().filter_map(|v| v))
        }));

        self
    }

    #[inline]
    pub fn body(mut self, body: Dom) -> Self {
        self.data.body = Some(body);
        self
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
                _ => None,
            },
            match self.body {
                Some(body) => Some(html!("div", {
                    .class("body")
                    .class("card-section")
                    .child(body)
                })),
                _ => None,
            },
            match self.footer {
                Some(footer) => Some(html!("div", {
                    .class("footer")
                    .class("card-section")
                    .child(footer)
                })),
                _ => None,
            },
        ];

        html!("div", {
            .class("dmat-card")
            .children(children.into_iter().filter_map(|v| v))
        })
    }
}
