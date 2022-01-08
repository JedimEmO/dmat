use crate::theme::colors::Colors;
use crate::theme::components::Components;
use crate::theme::to_sass::{render_sass_map, SassProperty, ToSass};

#[derive(Default)]
pub struct DmatTheme {
    pub colors: Colors,
    pub components: Components,
}

impl ToSass for DmatTheme {
    fn to_sass(&self) -> String {
        render_sass_map(vec![
            SassProperty {
                name: "colors".to_string(),
                value: self.colors.clone(),
            }
            .to_sass(),
            SassProperty {
                name: "components".to_string(),
                value: self.components.clone(),
            }
            .to_sass(),
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
