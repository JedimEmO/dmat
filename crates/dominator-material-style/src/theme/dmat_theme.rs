use crate::theme::colors::Colors;
use crate::theme::components::Components;
use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

#[derive(Default)]
pub struct DmatTheme {
    pub colors: Colors,
    pub components: Components,
}

impl ToSass for DmatTheme {
    fn to_sass(&self) -> String {
        render_sass_map(vec![
            render_sass_property("colors", &self.colors),
            render_sass_property("components", &self.colors),
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
