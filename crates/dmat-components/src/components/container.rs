use dominator::{html, Dom};

#[component(render_fn = container)]
pub struct Container {
    #[signal_vec]
    #[default(vec![])]
    children: Dom,
}

pub fn container(props: impl ContainerPropsTrait + 'static) -> Dom {
    let ContainerProps { children, apply } = props.take();

    html!("div", {
        .class("dmat-container")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .children_signal_vec(children)
    })
}
