use dominator::{html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;

#[component(render_fn = title)]
pub struct Title {
    #[signal]
    #[default(String::new())]
    pub header_text: String,
    #[signal]
    #[default(None)]
    pub sub_header_text: Option<String>,
}

pub fn title(props: impl TitlePropsTrait + 'static) -> Dom {
    let TitleProps {
        header_text,
        sub_header_text,
        apply,
    } = props.take();

    let children = map_ref! {
        let header = header_text,
        let sub_header = sub_header_text => move {
            vec![
                Some(html!("div", {
                    .class("title")
                    .text(header)
                })),
                sub_header.as_ref().map(move |v| html!("div", {
                    .class("sub-title")
                    .text(v)
                }))
            ].into_iter().flatten().collect()
        }
    }
    .to_signal_vec();

    html!("div",{
        .class("dmat-title")
        .apply_if(apply.is_some(), |d| d.apply(apply.unwrap()))
        .children_signal_vec(children)
    })
}
