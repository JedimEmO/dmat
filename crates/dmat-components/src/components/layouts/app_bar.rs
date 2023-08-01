use dominator::{html, Dom};
use futures_signals::signal::SignalExt;

#[component(render_fn = app_bar)]
pub struct AppBar {
    #[signal]
    #[default(None)]
    pub main: Option<Dom>,
    #[signal]
    #[default(None)]
    pub header: Option<Dom>,
    #[signal]
    #[default(AppBarType::Normal)]
    pub app_bar_type: AppBarType,
    #[signal]
    #[default(false)]
    pub fixed: bool,
}

#[inline]
pub fn app_bar(props: impl AppBarPropsTrait + 'static) -> Dom {
    let AppBarProps {
        main,
        header,
        app_bar_type,
        fixed,
        apply,
    } = props.take();

    let type_bc = app_bar_type.broadcast();

    html!("div", {
        .class("dmat-app-bar")
        .apply_if(apply.is_some(), |dom| dom.apply(apply.unwrap()))
        .class_signal("-fixed", fixed)
        .child_signal(header.map(|header_view| {
            Some(html!("header", {
                .class("header")
                .apply_if(header_view.is_some(), |d| d.child(header_view.unwrap()))
            }))
        }))
        .child_signal(main.map(move |main_view| {
            Some(html!("main", {
                .class("main")
                .class_signal("-normal", type_bc.signal_cloned().map(|t| t == AppBarType::Normal))
                .class_signal("-prominent", type_bc.signal_cloned().map(|t| t == AppBarType::Prominent))
                .apply_if(main_view.is_some(), |d| d.child(main_view.unwrap()))
            }))
        }))
    })
}

#[derive(Clone, Default, PartialEq)]
pub enum AppBarType {
    #[default]
    Normal,
    Prominent,
}
