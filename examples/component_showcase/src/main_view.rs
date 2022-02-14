use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;

use dominator_material::components::layouts::{app_bar, AppBarProps};
use dominator_material::components::TabsProps;
use dominator_material::utils::mixin::{id_attribute_mixin, stream_handler_mixin};

use crate::demo_views::about::about_view;
use crate::demo_views::component_demo::component_demo_view;
use crate::route::{DemoRoute, ExampleAppRoute};

pub fn main_view() -> Dom {
    let active_tab = ExampleAppRoute::signal();

    let (menu_tabs, menu_tabs_out) = tabs!(TabsProps {
        tab_render_fn: render_top_level_tabs,
        is_active_tab_signal_factory: |id| ExampleAppRoute::signal().map(clone!(id => move |v| {
                // This is a tad funky, since the second tab is a collection of multiple possible enum values
                if id == ExampleAppRoute::About {
                    return v == id
                } else {
                    return v != ExampleAppRoute::About
                }
        })),
        tabs_list: MutableVec::new_with_values(vec![
            ExampleAppRoute::About,
            ExampleAppRoute::Components(DemoRoute::AppBar)
        ])
        .signal_vec()
    });

    app_bar(
        AppBarProps::new()
            .header(html!("div", {
                .children(&mut [
                    html!("h1", {
                       .text("Dominator Material")
                    }),
                    menu_tabs
                ])
            }))
            .main(main_app_view(active_tab))
            .fixed(),
        |d| {
            d.apply(id_attribute_mixin("dmat-example-app"))
                .apply(stream_handler_mixin(
                    menu_tabs_out.tab_select_stream,
                    |new_tab| ExampleAppRoute::goto(new_tab),
                ))
        },
    )
}

fn main_app_view<S: Signal<Item = ExampleAppRoute> + 'static>(active_route: S) -> Dom {
    html!("div", {
        .child_signal(active_route.map(|route| {
            match route {
                ExampleAppRoute::About => Some(about_view()),
                ExampleAppRoute::Components(c) => Some(component_demo_view(c)),
            }
        }))
    })
}

fn render_top_level_tabs(route: ExampleAppRoute) -> Dom {
    match route {
        ExampleAppRoute::About => text!("About"),
        ExampleAppRoute::Components(_) => text!("Components"),
    }
}
