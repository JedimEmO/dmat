use dominator::{html, Dom};
use futures_signals::signal::{always, MutableSignal};
use futures_signals::signal::{Mutable, SignalExt};

use dominator_material::components::layouts::{AppBarType, Container};
use dominator_material::components::{
    card, layouts::AppBar, CardProps, Carousel, CarouselSource, NavigationDrawer,
    NavigationDrawerEntry, NavigationEntry,
};
use dominator_material::utils::renderable_child::IntoRenderableChild;

use crate::components::navigation_drawer_demo::NavigationDrawerDemo;

pub struct AppBarDemo {}

impl AppBarDemo {
    pub fn new() -> AppBarDemo {
        AppBarDemo {}
    }

    pub fn render(self) -> Dom {
        card(
            CardProps::new()
                .with_apply(|v| v.class("demo-card"))
                .with_body(
                    Carousel::new(AppBarCarousel::new())
                        .render_apply(|d, _| d.class("demo-carousel")),
                ),
        )
    }
}

struct AppBarCarousel {
    count: Mutable<usize>,
}

impl AppBarCarousel {
    fn new() -> AppBarCarousel {
        AppBarCarousel {
            count: Mutable::new(3),
        }
    }
}

impl CarouselSource for AppBarCarousel {
    fn get_entry(&self, index: usize) -> Dom {
        let inner = match index {
            0 => NavigationDrawer::new()
                .apply(|_, dom| dom.class("demo-drawer-with-app-bar"))
                .show_toggle_controls(true)
                .expanded(true)
                .initial_selected(0)
                .entries(vec![
                    NavigationDrawerEntry::Item(NavigationEntry {
                        id: 0,
                        text: "Inbox".to_string(),
                    }),
                    NavigationDrawerEntry::Item(NavigationEntry {
                        id: 1,
                        text: "Spam".to_string(),
                    }),
                ])
                .title_view_generator(|_, _| Some(html!("div", {.text("Outer modal drawer")})))
                .modal(true)
                .main_view_generator(|_, _| {
                    Some(
                        AppBar::new()
                            .header(
                                html!("div", {
                                    .class("app-bar-demo-header")
                                })
                                .into_renderable_child(),
                            )
                            .main(
                                Container::new(
                                    html!("div", { .text(lipsum::lipsum(512).as_str())}),
                                )
                                .render()
                                .into_renderable_child(),
                            )
                            .fixed()
                            .render(),
                    )
                })
                .render(),
            1 => AppBar::new()
                .header(
                    html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Normal unfixed app bar")
                    })
                    .into_renderable_child(),
                )
                .main(always(true).map(|_| {
                    Some(html!("div", {
                        .text(lipsum::lipsum(1024).as_str())
                    }))
                }))
                .render(),
            _ => AppBar::new()
                .header(
                    html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    })
                    .into_renderable_child(),
                )
                .main(always(true).map(|_| Some(NavigationDrawerDemo::static_drawers(true))))
                .bar_type(AppBarType::Prominent)
                .fixed()
                .render(),
        };

        html!("div", {
            .class("showcase-drawer-carousel-inner")
            .child(inner)
        })
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
