use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

pub struct Inputs {
    pub border_radius: String,
    pub border: String,
    pub input_height: String,
    pub input_height_with_help_text: String,
}

impl ToSass for Inputs {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("border_radius", &self.border_radius),
            render_sass_property("border", &self.border),
            render_sass_property("input_height", &self.input_height),
            render_sass_property(
                "input_height_with_help_text",
                &self.input_height_with_help_text,
            ),
        ])
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            border_radius: "5px".to_string(),
            border: "1px solid black".to_string(),
            input_height: "45px".to_string(),
            input_height_with_help_text: "30px".to_string(),
        }
    }
}
