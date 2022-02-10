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

impl DemoRoute {
    pub fn new(url: Url) -> Self {
        match url.hash().as_str() {
            "#/appbar" => DemoRoute::AppBar,
            "#/button" => DemoRoute::Button,
            "#/list" => DemoRoute::List,
            "#/carousel" => DemoRoute::Carousel,
            "#/card" => DemoRoute::Card,
            "#/dock-overlay" => DemoRoute::DockOverlay,
            "#/tabs" => DemoRoute::Tabs,
            "#/data-table" => DemoRoute::DataTable,
            "#/input" => DemoRoute::Input,
            "#/navigation-drawer" => DemoRoute::NavigationDrawer,
            "#/sheet" => DemoRoute::Sheet,
            _ => DemoRoute::AppBar,
        }
    }

    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
            .signal_ref(|url| Url::new(url).unwrap_throw())
            .map(Self::new)
    }

    pub fn goto(route: Self) {
        dominator::routing::go_to_url(route.url());
    }

    fn url(&self) -> &str {
        match self {
            DemoRoute::AppBar => "#/appbar",
            DemoRoute::Button => "#/button",
            DemoRoute::List => "#/list",
            DemoRoute::Carousel => "#/carousel",
            DemoRoute::Card => "#/card",
            DemoRoute::DockOverlay => "#/dock-overlay",
            DemoRoute::Tabs => "#/tabs",
            DemoRoute::DataTable => "#/data-table",
            DemoRoute::Input => "#/input",
            DemoRoute::Sheet => "#/sheet",
            DemoRoute::NavigationDrawer => "#/navigation-drawer",
        }
    }
}
