use dmat_utils::svg::animated_attribute::animated_attribute;
use dominator::__internal::SvgElement;
use dominator::{clone, Dom, DomBuilder};
use futures_signals::signal::{always, Signal};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use std::time::Duration;

use super::axis::layout_axis;

#[macro_export]
macro_rules! line_chart {
    ($props: expr, $datasets: expr, $mixin: expr) => {{
        $crate::contexts::charts::line_chart::line_chart($props, $datasets, $mixin)
    }};
    ($props: expr, $datasets: expr) => {{
        $crate::contexts::charts::line_chart::line_chart($props, $datasets, |v| v)
    }};
}

#[derive(Clone)]
pub enum GraphColor {
    RGB { r: u8, g: u8, b: u8 },
}

impl GraphColor {
    pub(crate) fn to_css_stroke(&self) -> String {
        match self {
            GraphColor::RGB { r, g, b } => format!("rgb({},{},{})", r, g, b),
        }
    }
}

#[derive(Clone)]
pub enum DatasetValues {
    Static(Vec<Point>),
    Signal(Rc<dyn Fn() -> Box<dyn Signal<Item = Vec<Point>> + Unpin>>),
}

impl DatasetValues {
    pub fn to_signal(&self) -> Box<dyn Signal<Item = Vec<Point>> + Unpin> {
        match self {
            Self::Signal(signal_fn) => signal_fn(),
            Self::Static(static_data) => Box::new(always(static_data.clone())),
        }
    }
}

/// One set of data to be rendered within a chart
#[derive(Clone)]
pub struct LineDataset {
    pub values: DatasetValues,
    pub label: String,
    pub shaded: bool,
    pub color: GraphColor,
}

pub struct TickInfo {
    pub count: usize,
    pub format: fn(f32) -> String,
}

#[derive(Default)]
pub struct AxisDescription {
    pub min: f32,
    pub max: f32,
    pub ticks: Option<TickInfo>,
}

pub struct LineChartProps {
    pub x_axis: AxisDescription,
    pub y_axis: AxisDescription,
    pub width_px: usize,
    pub height_px: usize,
}

#[derive(Clone)]
pub struct ViewBox {
    pub data_min: Point,
    pub data_max: Point,
    pub view_width: f32,
    pub view_height: f32,
}

impl ViewBox {
    pub(crate) fn data_point_to_view_box_point(&self, data_point: &Point) -> Point {
        let scale_x = self.view_width / (self.data_max.x - self.data_min.x);
        let scale_y = self.view_height / (self.data_max.y - self.data_min.y);

        Point {
            x: (data_point.x - self.data_min.x) * scale_x,
            y: self.view_height - (data_point.y - self.data_min.y) * scale_y,
        }
    }
}

pub fn line_chart(
    props: LineChartProps,
    datasets: impl SignalVecExt<Item = LineDataset> + 'static,
    mixin: impl FnOnce(DomBuilder<SvgElement>) -> DomBuilder<SvgElement>,
) -> Dom {
    let view_box = ViewBox {
        data_min: Point {
            x: props.x_axis.min,
            y: props.y_axis.min,
        },
        data_max: Point {
            x: props.x_axis.max,
            y: props.y_axis.max,
        },
        view_width: props.width_px as f32,
        view_height: props.height_px as f32,
    };

    svg!("svg", {
        .apply(mixin)
        .attr("width", format!("{}px", props.width_px).as_str())
        .attr("height", format!("{}px", props.height_px).as_str())
        .attr("viewBox", format!("-20 -20 {} {}", props.width_px + 40, props.height_px + 40).as_str())
        .child(svg!("g", {
            .children_signal_vec(datasets.map(clone!(view_box => move |dataset| draw_data_set(dataset, view_box.clone()))))
        }))
        .child(layout_axis(&props.x_axis, &props.y_axis, &view_box))
    })
}

// move to shared code

fn line_points(points: &Vec<Point>, view_box: ViewBox) -> String {
    points
        .iter()
        .map(|v| {
            let view_point = view_box.data_point_to_view_box_point(v);
            format!("{},{}", view_point.x, view_point.y)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn draw_data_set(dataset: LineDataset, view_box: ViewBox) -> Dom {
    let points_signal = dataset.values.to_signal();

    svg!("g", {
        .apply(|builder| {
            if dataset.shaded {
                return builder.child(svg!("polygon", {
                    .apply(clone!(view_box => move |builder| {
                        animated_attribute(
                            builder,
                            dataset.values.to_signal(),
                            Rc::new(clone!(view_box => move |data: Vec<Point>|  {
                                let left_x = view_box.data_point_to_view_box_point(&data[0]).x;
                                let right_x = view_box.data_point_to_view_box_point(&data[data.len() - 1]).x;
                                let points_attr = line_points(&data, view_box.clone());
                                format!("{} {},{} {},{}", points_attr, right_x, view_box.view_height, left_x, view_box.view_height)
                            })),
                            "points".to_string(),
                            Duration::from_millis(200))
                    }))
                    .attr("fill", dataset.color.to_css_stroke().as_str())
                    .attr("opacity", "30%")
                }))
            }

            builder
        })

        .child(svg!("polyline", {
            .apply(clone!(view_box => move |builder| {
                animated_attribute(
                    builder,
                    points_signal,
                    Rc::new(clone!(view_box => move |data: Vec<Point>|  {
                        line_points(&data, view_box.clone())
                    })),
                    "points".to_string(),
                    Duration::from_millis(200))
            }))
            .attr("fill", "none")
            .attr("stroke", dataset.color.to_css_stroke().as_str() )
            .attr("stroke-width", "1")
        }))
    })
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
