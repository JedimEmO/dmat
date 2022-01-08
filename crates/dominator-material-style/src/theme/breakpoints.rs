use crate::theme::layout::DisplayUnit;
use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

pub struct Breakpoints {
    pub small: DisplayUnit,
    pub medium: DisplayUnit,
    pub large: DisplayUnit,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Breakpoints {
            small: DisplayUnit::Pixels(599),
            medium: DisplayUnit::Pixels(900),
            large: DisplayUnit::Pixels(1200),
        }
    }
}

impl ToSass for Breakpoints {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("small", &self.small),
            render_sass_property("medium", &self.medium),
            render_sass_property("large", &self.large),
        ])
    }
}

pub struct BreakpointValue<T: ToSass> {
    pub small: T,
    pub medium: T,
    pub large: T,
}

impl<T: ToSass> ToSass for BreakpointValue<T> {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("small", &self.small),
            render_sass_property("medium", &self.medium),
            render_sass_property("large", &self.large),
        ])
    }
}
