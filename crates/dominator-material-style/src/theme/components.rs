use crate::theme::to_sass::ToSass;

#[derive(Default, Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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
