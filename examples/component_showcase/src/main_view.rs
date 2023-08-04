use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

use dmat_components::components::*;
use dmat_components::utils::mixin::id_attribute_mixin;

use crate::demo_views::about::about_view;
use crate::demo_views::component_demo::component_demo_view;
use crate::route::ExampleAppRoute;
use crate::vis_components::line_chart_demo::line_chart_demo;

pub fn main_view() -> Dom {
    let active_tab = ExampleAppRoute::signal();

    let tabs_dom = tabs(
        TabsProps::new()
            .active_tab_signal(ExampleAppRoute::signal().map(|active_tab| {
                Some(match active_tab {
                    ExampleAppRoute::About => 0,
                    ExampleAppRoute::Components => 1,
                    ExampleAppRoute::VisComponents => 2,
                })
            }))
            .tab_click_handler(|idx| match idx {
                1 => ExampleAppRoute::goto(ExampleAppRoute::Components),
                2 => ExampleAppRoute::goto(ExampleAppRoute::VisComponents),
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
        .header(Some(html!("div", {
            .children(&mut [
                html!("h1", {
                   .text("Dmat Examples")
                }),
                tabs_dom
            ])
        })))
        .main(Some(main_app_view(active_tab)))
        .fixed(true)
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
                ExampleAppRoute::Components => Some(component_demo_view()),
                ExampleAppRoute::VisComponents => Some(line_chart_demo())
            }
        }))
    })
}
