use crate::theme::breakpoints::BreakpointValue;
use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

pub enum DisplayUnit {
    Pixels(i32),
}

pub struct Layout {
    pub margin: BreakpointValue<DisplayUnit>,
    pub gutter: BreakpointValue<DisplayUnit>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            margin: BreakpointValue {
                small: DisplayUnit::Pixels(8),
                medium: DisplayUnit::Pixels(16),
                large: DisplayUnit::Pixels(32),
            },
            gutter: BreakpointValue {
                small: DisplayUnit::Pixels(8),
                medium: DisplayUnit::Pixels(16),
                large: DisplayUnit::Pixels(24),
            },
        }
    }
}

impl ToSass for Layout {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("margin", &self.margin),
            render_sass_property("gutter", &self.gutter),
        ])
    }
}

impl ToSass for DisplayUnit {
    fn to_sass(&self) -> String {
        match self {
            DisplayUnit::Pixels(px) => format!("{}px", px),
        }
    }
}
