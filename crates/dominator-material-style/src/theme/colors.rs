use crate::theme::to_sass::{render_sass_map, SassProperty, ToSass};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Colors {
    pub primary: SassProperty<Color>,
    pub primary_light: SassProperty<Color>,
    pub primary_dark: SassProperty<Color>,
    pub text_on_primary: SassProperty<Color>,
    pub surface: SassProperty<Color>,
    pub surface_dark: SassProperty<Color>,
    pub secondary: SassProperty<Color>,
    pub secondary_dark: SassProperty<Color>,
    pub secondary_light: SassProperty<Color>,
    pub secondary_lightest: SassProperty<Color>,
    pub text_on_secondary: SassProperty<Color>,
    pub text_diffuse: SassProperty<Color>,
    pub text_diffuser: SassProperty<Color>,
    pub error_text_color: SassProperty<Color>,
    pub invalid_element_backdrop: SassProperty<Color>,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: ("primary", Color::Hex("#263238".to_string())).into(),
            primary_light: ("primary_light", Color::Hex("#4f5b62".to_string())).into(),
            primary_dark: ("primary_dark", Color::Hex("#000a12".to_string())).into(),
            text_on_primary: ("text_on_primary", Color::Hex("#ffffff".to_string())).into(),
            surface: ("surface", Color::Hex("#ffffff".to_string())).into(),
            surface_dark: ("surface_dark", Color::RGBA(47, 47, 47, 0.67)).into(),
            secondary: ("secondary", Color::Hex("#448aff".to_string())).into(),
            secondary_dark: ("secondary_dark", Color::Hex("#005ecb".to_string())).into(),
            secondary_light: ("secondary_light", Color::Hex("#83b9ff".to_string())).into(),
            secondary_lightest: ("secondary_lightest", Color::Hex("#d0e4fd".to_string())).into(),
            text_on_secondary: ("text_on_secondary", Color::Hex("#000000".to_string())).into(),
            text_diffuse: ("text_diffuse", Color::Hex("#474747".to_string())).into(),
            text_diffuser: ("text_diffuser", Color::Hex("#787878".to_string())).into(),
            error_text_color: ("error_text_color", Color::Hex("#b00020".to_string())).into(),
            invalid_element_backdrop: (
                "invalid_element_backdrop",
                Color::Hex("#ffb3a9".to_string()),
            )
                .into(),
        }
    }
}

impl ToSass for Colors {
    fn to_sass(&self) -> String {
        let colors: Vec<String> = vec![
            self.primary.to_sass(),
            self.primary_light.to_sass(),
            self.primary_dark.to_sass(),
            self.text_on_primary.to_sass(),
            self.surface.to_sass(),
            self.surface_dark.to_sass(),
            self.secondary.to_sass(),
            self.secondary_dark.to_sass(),
            self.secondary_light.to_sass(),
            self.secondary_lightest.to_sass(),
            self.text_on_secondary.to_sass(),
            self.text_diffuse.to_sass(),
            self.text_diffuser.to_sass(),
            self.error_text_color.to_sass(),
            self.invalid_element_backdrop.to_sass(),
        ]
        .into_iter()
        .map(|c| format!("\t\t{}", c))
        .collect();

        render_sass_map(colors)
    }
}
