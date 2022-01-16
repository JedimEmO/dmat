use crate::theme::to_sass::{render_sass_map, render_sass_property, ToSass};

#[derive(Default)]
pub struct Components {
    pub app_bar: AppBar,
    pub navigation_drawer: NavigationDrawer,
}

impl ToSass for Components {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("app_bar", &self.app_bar),
            render_sass_property("navigation_drawer", &self.navigation_drawer),
        ])
    }
}

pub struct AppBar {
    pub height: String,
    pub height_prominent: String,
}

impl Default for AppBar {
    fn default() -> Self {
        Self {
            height: "40px".to_string(),
            height_prominent: "80px".to_string(),
        }
    }
}

impl ToSass for AppBar {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("height", &self.height),
            render_sass_property("height_prominent", &self.height_prominent),
        ])
    }
}

pub struct NavigationDrawer {
    pub full_width: String,
    pub narrow_width: String,
}

impl Default for NavigationDrawer {
    fn default() -> Self {
        Self {
            full_width: "150px".to_string(),
            narrow_width: "50px".to_string(),
        }
    }
}

impl ToSass for NavigationDrawer {
    fn to_sass(&self) -> String {
        render_sass_map(&[
            render_sass_property("full_width", &self.full_width),
            render_sass_property("narrow_width", &self.narrow_width),
        ])
    }
}
