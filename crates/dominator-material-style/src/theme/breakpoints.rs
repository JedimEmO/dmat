use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

pub struct BreakpointValue<T: ToSass> {
    pub small: T,
    pub medium: T,
    pub large: T,
}

impl<T: ToSass> ToSass for BreakpointValue<T> {
    fn to_sass(&self) -> String {
        render_sass_map(vec![
            render_sass_property("small", &self.small),
            render_sass_property("medium", &self.medium),
            render_sass_property("large", &self.large),
        ])
    }
}
