use crate::theme::colors::Colors;
use crate::theme::components::Components;
use crate::theme::to_sass::ToSass;

#[derive(Default)]
pub struct DmatTheme {
    pub colors: Colors,
    pub components: Components,
}

impl ToSass for DmatTheme {
    fn to_sass(&self) -> String {
        format!(
            "(\
            \n\t\"colors\": {},\
            \n\t\"components\": {}\n\t)",
            self.colors.to_sass(),
            self.components.to_sass()
        )
    }
}

pub fn render_theme(theme_name: &str, theme: DmatTheme) -> String {
    format!(
        "// Auto generated dmat theme file \n${}:{};",
        theme_name,
        theme.to_sass()
    )
}
