use crate::theme::colors::Colors;
use crate::theme::components::Components;
use crate::theme::layout::Layout;
use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

#[derive(Default)]
pub struct DmatTheme {
    pub colors: Colors,
    pub components: Components,
    pub layout: Layout,
}

impl ToSass for DmatTheme {
    fn to_sass(&self) -> String {
        render_sass_map(vec![
            render_sass_property("colors", &self.colors),
            render_sass_property("components", &self.components),
            render_sass_property("layout", &self.layout),
        ])
    }
}

pub fn render_theme(theme_name: &str, theme: DmatTheme) -> String {
    format!(
        "// Auto generated dmat theme file \n${}:{};",
        theme_name,
        theme.to_sass()
    )
}
