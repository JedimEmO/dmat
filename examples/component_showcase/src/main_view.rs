use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use futures_signals::signal_vec::MutableVec;

use dmat_components::components::layouts::{app_bar, AppBarProps};
use dmat_components::components::TabsProps;
use dmat_components::utils::mixin::{id_attribute_mixin, stream_handler_mixin};

use crate::demo_views::about::about_view;
use crate::demo_views::component_demo::component_demo_view;
use crate::route::{DemoRoute, ExampleAppRoute, VisDemoRoute};
use crate::vis_components::line_chart_demo::line_chart_demo;

pub fn main_view() -> Dom {
    let active_tab = ExampleAppRoute::signal();

    let (menu_tabs, menu_tabs_out) = tabs!(TabsProps {
        tab_render_fn: render_top_level_tabs,
        is_active_tab_signal_factory: |id| ExampleAppRoute::signal().map(clone!(id => move |v| {
               v.is_same_category(id)
        })),
        tabs_list: MutableVec::new_with_values(vec![
            ExampleAppRoute::About,
            ExampleAppRoute::Components(DemoRoute::AppBar),
            ExampleAppRoute::VisComponents(VisDemoRoute::LineChart)
        ])
        .signal_vec()
    });

    app_bar(
        AppBarProps::new()
            .header(html!("div", {
                .children(&mut [
                    html!("h1", {
                       .text("Dmat Examples")
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
                    ExampleAppRoute::goto,
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
                ExampleAppRoute::VisComponents(_) => Some(line_chart_demo())
            }
        }))
    })
}

fn render_top_level_tabs(route: ExampleAppRoute) -> Dom {
    match route {
        ExampleAppRoute::About => text!("About"),
        ExampleAppRoute::Components(_) => text!("Components"),
        ExampleAppRoute::VisComponents(_) => text!("Visualization Components"),
    }
}
