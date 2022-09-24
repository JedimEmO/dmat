use dominator::routing;
use futures_signals::signal::{Signal, SignalExt};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Url;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DemoRoute {
    AppBar,
    Button,
    List,
    Carousel,
    Card,
    DockOverlay,
    Tabs,
    DataTable,
    Input,
    NavigationDrawer,
    Sheet,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum VisDemoRoute {
    LineChart,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExampleAppRoute {
    Components(DemoRoute),
    VisComponents(VisDemoRoute),
    About,
}

impl ExampleAppRoute {
    pub fn new(url: Url) -> Self {
        let url_value = url.hash();

        if url_value.as_str().contains("#/about") {
            ExampleAppRoute::About
        } else if url_value.as_str().contains("#/component/") {
            ExampleAppRoute::Components(DemoRoute::new(url_value.as_str()))
        } else if url_value.as_str().contains("#/vis-component/") {
            ExampleAppRoute::VisComponents(VisDemoRoute::LineChart)
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
            Self::Components(c) => format!("#/component/{}", c.url()),
            Self::VisComponents(c) => format!("#/vis-components/{}", c.url()),
        }
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(route.url().as_str());
    }

    pub fn is_same_category(&self, other: Self) -> bool {
        match self {
            Self::About => match other {
                Self::About => true,
                _ => false,
            },
            Self::VisComponents(_) => match other {
                Self::VisComponents(_) => true,
                _ => false,
            },
            Self::Components(_) => match other {
                Self::Components(_) => true,
                _ => false,
            },
        }
    }
}

impl DemoRoute {
    pub fn new(url: &str) -> Self {
        match url {
            "#/component/appbar" => DemoRoute::AppBar,
            "#/component/button" => DemoRoute::Button,
            "#/component/list" => DemoRoute::List,
            "#/component/carousel" => DemoRoute::Carousel,
            "#/component/card" => DemoRoute::Card,
            "#/component/dock-overlay" => DemoRoute::DockOverlay,
            "#/component/tabs" => DemoRoute::Tabs,
            "#/component/data-table" => DemoRoute::DataTable,
            "#/component/input" => DemoRoute::Input,
            "#/component/navigation-drawer" => DemoRoute::NavigationDrawer,
            "#/component/sheet" => DemoRoute::Sheet,
            _ => DemoRoute::AppBar,
        }
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(route.url());
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
            DemoRoute::DataTable => "data-table",
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
