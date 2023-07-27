use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

use dmat_components::components::{tabs, TabsProps};
use dmat_components::utils::mixin::id_attribute_mixin;

use crate::demo_views::about::about_view;
use crate::demo_views::component_demo::component_demo_view;
use crate::route::{DemoRoute, ExampleAppRoute, VisDemoRoute};
use crate::vis_components::line_chart_demo::line_chart_demo;

pub fn main_view() -> Dom {
    let active_tab = ExampleAppRoute::signal();

    let tabs_dom = tabs(
        TabsProps::new()
            .active_tab_signal(ExampleAppRoute::signal().map(|active_tab| {
                Some(match active_tab {
                    ExampleAppRoute::About => 0,
                    ExampleAppRoute::Components(_) => 1,
                    ExampleAppRoute::VisComponents(_) => 2,
                })
            }))
            .tab_click_handler(|idx| match idx {
                1 => ExampleAppRoute::goto(ExampleAppRoute::Components(DemoRoute::Button)),
                2 => ExampleAppRoute::goto(ExampleAppRoute::VisComponents(VisDemoRoute::LineChart)),
                _ => ExampleAppRoute::goto(ExampleAppRoute::About),
            })
            .tabs(vec![
                html!("div", {
                    .text("About")
                }),
                html!("div", {
                    .text("Components")
                }),
                html!("div", {
                    .text("Visualization Components")
                }),
            ]),
    );

    app_bar!({
        .header(html!("div", {
            .children(&mut [
                html!("h1", {
                   .text("Dmat Examples")
                }),
                tabs_dom
            ])
        }))
        .main(main_app_view(active_tab))
        .fixed()
        .apply(|d| {
            d.apply(id_attribute_mixin("dmat-example-app"))
        })
    })
}

fn main_app_view<S: Signal<Item = ExampleAppRoute> + 'static>(active_route: S) -> Dom {
    html!("div", {
        .child_signal(active_route.map(|route| {
            match route {
                ExampleAppRoute::About => Some(about_view()),
                ExampleAppRoute::Components(c) => Some(component_demo_view(c)),
                ExampleAppRoute::VisComponents(_) => Some(line_chart_demo())
            }
        }))
    })
}
