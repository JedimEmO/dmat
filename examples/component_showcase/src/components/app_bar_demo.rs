use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;

use crate::components::navigation_drawer_demo::static_drawers;
use dominator_material::components::layouts::{app_bar, AppBarProps, AppBarType};
use dominator_material::components::{
    CardProps, CarouselProps, CarouselSource, DrawerWidth, NavigationDrawerEntry,
    NavigationDrawerProps,
};
use dominator_material::utils::mixin::mixin_id;

pub fn app_bar_demo() -> Dom {
    card!(CardProps::new().body(
        carousel!(
            CarouselProps {
                source: AppBarCarousel::new(),
                initial_view_index: Default::default(),
            },
            |d| d.class("demo-carousel")
        )
        .0,
    ))
}

#[derive(Clone)]
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

#[derive(Clone, Copy, PartialEq)]
enum DemoNavigationEntries {
    Inbox,
    Spam,
}

impl CarouselSource for AppBarCarousel {
    fn get_entry(&self, index: usize) -> Dom {
        let inner = match index {
            0 => {
                navigation_drawer!(
                    NavigationDrawerProps::new(
                        |_active_view| {
                            Some(app_bar(
                                AppBarProps::new()
                                    .header(html!("div", {
                                        .class("app-bar-demo-header")
                                    }))
                                    .main(container!(|d| d.child(
                                        html!("div", { .text(lipsum::lipsum(512).as_str())})
                                    )))
                                    .fixed(),
                                mixin_id(),
                            ))
                        },
                        |item, _expanded| {
                            match item {
                                DemoNavigationEntries::Inbox => text!("inbox"),
                                DemoNavigationEntries::Spam => text!("spam"),
                            }
                        }
                    )
                    .show_toggle_controls(true)
                    .expanded(true)
                    .initial_selected(DemoNavigationEntries::Inbox)
                    .entries(vec![
                        NavigationDrawerEntry::Item(DemoNavigationEntries::Inbox),
                        NavigationDrawerEntry::Item(DemoNavigationEntries::Spam),
                    ])
                    .header_view_generator(|_, _| {
                        Some(html!("div", {.text("Outer modal drawer")}))
                    })
                    .modal(true),
                    |d| d.class("demo-drawer-with-app-bar")
                )
                .0
            }
            1 => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Normal unfixed app bar")
                    }))
                    .main(html!("div", {
                        .text(lipsum::lipsum(1024).as_str())
                    })),
                mixin_id(),
            ),
            _ => app_bar(
                AppBarProps::new()
                    .header(html!("h1", {
                        .class("app-bar-demo-header")
                        .text("Prominent fixed app bar")
                    }))
                    .main(static_drawers(true, DrawerWidth::Full))
                    .bar_type(AppBarType::Prominent)
                    .fixed(),
                mixin_id(),
            ),
        };

        container!(|d| d
            .child(inner)
            .style("padding", "16px")
            .style("position", "relative")
            .style("margin", "auto")
            .style("width", "500px")
            .style("height", "500px"))
    }

    fn total_count_signal(&self) -> MutableSignal<usize> {
        self.count.signal()
    }

    fn total_count(&self) -> usize {
        self.count.get()
    }
}
