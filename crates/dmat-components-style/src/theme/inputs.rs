use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

pub struct Inputs {
    pub border_radius: String,
    pub border: String,
    pub input_height: String,
    pub input_baseline: String,
    pub invalid_input_background: String,
}

impl ToSass for Inputs {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("border_radius", &self.border_radius),
            render_sass_property("border", &self.border),
            render_sass_property("input_height", &self.input_height),
            render_sass_property("input_baseline", &self.input_baseline),
            render_sass_property("invalid_input_background", &self.invalid_input_background),
        ])
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            border_radius: "5px".to_string(),
            border: "1px solid black".to_string(),
            input_height: "60px".to_string(),
            input_baseline: "45px".to_string(),
            invalid_input_background: "rgba(255, 100, 100, 0.1)".to_string(),
        }
    }
}
