use dominator::traits::AsStr;
use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

pub enum IconSize {
    Default,
    Small,
    Normal,
    Large,
    ExtraLarge,
}

#[inline]
/// Render an icon using iconfify
/// see https://iconify.design/ for more information on usage and available icon libraries
///
/// The apply function will be called with the containers DomBuilder, allowing
/// customization
///
/// icon_name must be a valid iconify selector, on the form {collection}:{icon-name}
pub fn icon<T: AsStr>(size: IconSize, icon_name: T) -> Dom {
    icon_with_apply(size, icon_name, |d| d)
}

#[inline]
/// Render an icon using iconfify
/// see https://iconify.design/ for more information on usage and available icon libraries
///
/// The apply function will be called with the containers DomBuilder, allowing
/// customization
/// icon_name must be a valid iconify selector, on the form {collection}:{icon-name}
pub fn icon_with_apply<F, T: AsStr>(size: IconSize, icon_name: T, apply: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
{
    let size_class = match size {
        IconSize::Default => "-default",
        IconSize::Normal => "-normal",
        IconSize::Large => "-large",
        IconSize::ExtraLarge => "-extra-large",
        IconSize::Small => "-small",
    };

    html!("span", {
        .class("iconify")
        .class("dmat-icon")
        .class(size_class)
        .apply(apply)
        .attribute("data-icon", icon_name.as_str().to_string().as_str())
        .attribute("data-inline", "false")
    })
}
