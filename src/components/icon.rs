use dominator::{html, Dom, DomBuilder};
use web_sys::HtmlElement;

pub enum IconSize {
    Default,
    Small,
    Normal,
    Large,
    ExtraLarge,
}

/// Render an icon using iconfify
/// see https://iconify.design/ for more information on usage and available icon libraries
pub struct Icon {
    size: IconSize,
    icon_name: String,
}

impl Icon {
    /// Construct a new Icon.
    ///
    /// icon_name must be a valid iconify selector, on the form {collection}:{icon-name}
    ///
    /// ```
    /// use dominator_material::components::{Icon, IconSize};
    /// let dom = Icon::new("ic::baseline-home", IconSize::Default).render();
    /// ```
    pub fn new<T1: ToString>(icon_name: T1, size: IconSize) -> Icon {
        Icon {
            size,
            icon_name: icon_name.to_string(),
        }
    }

    #[inline]
    pub fn render(self) -> Dom {
        self.render_apply(|v| v)
    }

    #[inline]
    /// Render the Icon
    ///
    /// The apply function will be called with the containers DomBuilder, allowing
    /// customization
    pub fn render_apply<F>(self, apply: F) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> + 'static,
    {
        let size_class = match self.size {
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
            .attribute("data-icon", format!("{}",self.icon_name ).as_str())
            .attribute("data-inline", "false")
        })
    }
}
