use dominator::routing;
use futures_signals::signal::{Signal, SignalExt};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DemoRoute {
    AppBar,
    Button,
    List,
    Carousel,
    Card,
    DockOverlay,
    Tabs,
    Table,
    Input,
    NavigationDrawer,
    Sheet,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VisDemoRoute {
    LineChart,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ExampleAppRoute {
    Components,
    VisComponents,
    About,
}

impl ExampleAppRoute {
    pub fn new(url: Url) -> Self {
        let url_value = url.hash();

        if url_value.as_str().contains("#/about") {
            ExampleAppRoute::About
        } else if url_value.as_str().contains("#/component/") {
            ExampleAppRoute::Components
        } else if url_value.as_str().contains("#/vis-component/") {
            ExampleAppRoute::VisComponents
        } else {
            ExampleAppRoute::About
        }
    }

    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
            .signal_ref(|url| Url::new(url).unwrap_throw())
            .map(Self::new)
    }

    pub fn url(&self) -> String {
        match self {
            Self::About => "#/about".to_string(),
            Self::Components => "#/component/".to_string(),
            Self::VisComponents => "#/vis-component/".to_string(),
        }
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(route.url().as_str());
    }
}

impl DemoRoute {
    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
            .signal_ref(|url| Url::new(url).unwrap_throw())
            .map(Self::new)
    }

    pub fn new(url: Url) -> Self {
        match url.hash().as_str() {
            "#/component/appbar" => DemoRoute::AppBar,
            "#/component/button" => DemoRoute::Button,
            "#/component/list" => DemoRoute::List,
            "#/component/carousel" => DemoRoute::Carousel,
            "#/component/card" => DemoRoute::Card,
            "#/component/dock-overlay" => DemoRoute::DockOverlay,
            "#/component/tabs" => DemoRoute::Tabs,
            "#/component/data-table" => DemoRoute::Table,
            "#/component/input" => DemoRoute::Input,
            "#/component/navigation-drawer" => DemoRoute::NavigationDrawer,
            "#/component/sheet" => DemoRoute::Sheet,
            _ => DemoRoute::AppBar,
        }
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(format!("#/component/{}", route.url()).as_str());
    }

    pub fn url(&self) -> &str {
        match self {
            DemoRoute::AppBar => "appbar",
            DemoRoute::Button => "button",
            DemoRoute::List => "list",
            DemoRoute::Carousel => "carousel",
            DemoRoute::Card => "card",
            DemoRoute::DockOverlay => "dock-overlay",
            DemoRoute::Tabs => "tabs",
            DemoRoute::Table => "data-table",
            DemoRoute::Input => "input",
            DemoRoute::Sheet => "sheet",
            DemoRoute::NavigationDrawer => "navigation-drawer",
        }
    }
}

impl VisDemoRoute {
    pub fn new(url: &str) -> Self {
        match url {
            "#/component/line-chart" => VisDemoRoute::LineChart,
            _ => VisDemoRoute::LineChart,
        }
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(route.url());
    }

    pub fn url(&self) -> &str {
        match self {
            VisDemoRoute::LineChart => "line-chart",
        }
    }
}
