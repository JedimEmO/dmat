use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

impl CardProps {
    pub fn new() -> Self {
        CardProps {
            ..Default::default()
        }
    }

    pub fn with_title<A: Into<String>>(mut self, title: A, sub_title: Option<A>) -> Self {
        self.header = Some(html!("div", {
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

    pub fn with_apply<F: 'static>(mut self, apply: F) -> Self
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        self.apply = Some(Box::new(apply));
        self
    }

    pub fn with_body(mut self, body: Dom) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_header(mut self, header: Dom) -> Self {
        self.header = Some(header);
        self
    }

    pub fn with_footer(mut self, footer: Dom) -> Self {
        self.footer = Some(footer);
        self
    }
}

#[derive(Default)]
pub struct CardProps {
    pub header: Option<Dom>,
    pub body: Option<Dom>,
    pub footer: Option<Dom>,
    pub apply: Option<Box<dyn FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>>,
}

pub fn card(props: CardProps) -> Dom {
    let mut apply = props.apply;
    let head = props.header;
    let body = props.body;
    let footer = props.footer;

    let children = vec![
        html!("div", {
            .class("dmat-card-header-container")
            .apply_if(head.is_some(), move |dom| {
                dom.child(head.unwrap())
            })
        }),
        html!("div", {
            .class("dmat-card-body-container")
            .apply_if(body.is_some(), move |dom| {
                dom.child(body.unwrap())
            })
        }),
        html!("div", {
            .class("dmat-card-footer-container")
            .apply_if(footer.is_some(), move |dom| {
                dom.child(footer.unwrap())
            })
        }),
    ];

    html!("div", {
        .class("dmat-card")
        .apply_if(apply.is_some(), move |dom| {
            dom.apply(apply.take().unwrap())
        })
        .children(children.into_iter())
    })
}
