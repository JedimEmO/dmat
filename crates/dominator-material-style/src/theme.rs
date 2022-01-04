pub trait ToSass {
    fn to_sass(&self) -> String;
}

pub fn render_theme(theme_name: &str, theme: DmatTheme) -> String {
    format!(
        "// Auto generated dmat theme file \n${}:{};",
        theme_name,
        theme.to_sass()
    )
}
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

pub enum Color {
    RGBA(u8, u8, u8, f32),
    RGB(u8, u8, u8),
    Hex(String),
}

impl Default for Color {
    fn default() -> Self {
        Color::RGB(0, 0, 0)
    }
}

impl ToSass for Color {
    fn to_sass(&self) -> String {
        match self {
            Color::RGBA(r, g, b, a) => format!("rgba({},{},{},{})", r, g, b, a),
            Color::RGB(r, g, b) => format!("rgba({},{},{})", r, g, b),
            Color::Hex(h) => h.clone(),
        }
    }
}

pub struct Colors {
    pub primary: Color,
    pub primary_light: Color,
    pub primary_dark: Color,
    pub text_on_primary: Color,
    pub surface: Color,
    pub surface_dark: Color,
    pub secondary: Color,
    pub secondary_dark: Color,
    pub secondary_light: Color,
    pub secondary_lightest: Color,
    pub text_on_secondary: Color,
    pub text_diffuse: Color,
    pub text_diffuser: Color,
    pub error_text_color: Color,
    pub invalid_element_backdrop: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: Color::Hex("#263238".to_string()),
            primary_light: Color::Hex("#4f5b62".to_string()),
            primary_dark: Color::Hex("#000a12".to_string()),
            text_on_primary: Color::Hex("#ffffff".to_string()),
            surface: Color::Hex("#ffffff".to_string()),
            surface_dark: Color::RGBA(47, 47, 47, 0.67),
            secondary: Color::Hex("#448aff".to_string()),
            secondary_dark: Color::Hex("#005ecb".to_string()),
            secondary_light: Color::Hex("#83b9ff".to_string()),
            secondary_lightest: Color::Hex("#d0e4fd".to_string()),
            text_on_secondary: Color::Hex("#000000".to_string()),
            text_diffuse: Color::Hex("#474747".to_string()),
            text_diffuser: Color::Hex("#787878".to_string()),
            error_text_color: Color::Hex("#b00020".to_string()),
            invalid_element_backdrop: Color::Hex("#ffb3a9".to_string()),
        }
    }
}

impl ToSass for Colors {
    fn to_sass(&self) -> String {
        let colors: Vec<String> = vec![
            format!("\"primary\": {}", self.primary.to_sass()),
            format!("\"primary_light\": {}", self.primary_light.to_sass()),
            format!("\"primary_dark\": {}", self.primary_dark.to_sass()),
            format!("\"text_on_primary\": {}", self.text_on_primary.to_sass()),
            format!("\"surface\": {}", self.surface.to_sass()),
            format!("\"surface_dark\": {}", self.surface_dark.to_sass()),
            format!("\"secondary\": {}", self.secondary.to_sass()),
            format!("\"secondary_dark\": {}", self.secondary_dark.to_sass()),
            format!("\"secondary_light\": {}", self.secondary_light.to_sass()),
            format!(
                "\"secondary_lightest\": {}",
                self.secondary_lightest.to_sass()
            ),
            format!(
                "\"text_on_secondary\": {}",
                self.text_on_secondary.to_sass()
            ),
            format!("\"text_diffuse\": {}", self.text_diffuse.to_sass()),
            format!("\"text_diffuser\": {}", self.text_diffuser.to_sass()),
            format!("\"error_text_color\": {}", self.error_text_color.to_sass()),
            format!(
                "\"invalid_element_backdrop\": {}",
                self.invalid_element_backdrop.to_sass()
            ),
        ]
        .into_iter()
        .map(|c| format!("\t\t{}", c))
        .collect();

        format!("(\n{})", colors.join(",\n"))
    }
}

#[derive(Default)]
pub struct Components {
    pub app_bar: AppBar,
    pub navigation_drawer: NavigationDrawer,
}

impl ToSass for Components {
    fn to_sass(&self) -> String {
        format!(
            "(\n\t\t\"app_bar\": {},\n\t\t\n\t\t\"navigation_drawer\": {})",
            self.app_bar.to_sass(),
            self.navigation_drawer.to_sass()
        )
    }
}

pub struct AppBar {
    pub height: String,
    pub height_prominent: String,
}

impl Default for AppBar {
    fn default() -> Self {
        Self {
            height: "2rem".to_string(),
            height_prominent: "4rem".to_string(),
        }
    }
}

impl ToSass for AppBar {
    fn to_sass(&self) -> String {
        let props: Vec<String> = vec![
            format!("\"height\": {}", self.height),
            format!("\"height_prominent\": {}", self.height_prominent),
        ]
        .into_iter()
        .map(|c| format!("\t\t{}", c))
        .collect();
        format!("(\n{})", props.join(",\n"))
    }
}

pub struct NavigationDrawer {
    pub full_width: String,
}

impl Default for NavigationDrawer {
    fn default() -> Self {
        Self {
            full_width: "15rem".to_string(),
        }
    }
}

impl ToSass for NavigationDrawer {
    fn to_sass(&self) -> String {
        let props: Vec<String> = vec![format!("\"full_width\": {}", self.full_width)]
            .into_iter()
            .map(|c| format!("\t\t{}", c))
            .collect();
        format!("(\n{})", props.join(",\n"))
    }
}

#[cfg(test)]
mod test {
    use crate::theme::{render_theme, DmatTheme};
    use grass::Options;

    #[test]
    fn verify_rendered_sass() {
        let theme = DmatTheme::default();
        let theme_file_content = render_theme("test-theme", theme);

        grass::from_string(theme_file_content, &Options::default()).unwrap();
    }
}
