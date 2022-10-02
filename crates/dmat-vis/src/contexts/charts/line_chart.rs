use dominator::__internal::SvgElement;
use dominator::{clone, Dom, DomBuilder};
use futures_signals::signal_vec::SignalVecExt;

use super::axis::{layout_axis, Axis, AxisOrientation};

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

/// One set of data to be rendered within a chart
#[derive(Clone)]
pub struct LineDataset {
    pub values: Vec<Point>,
    pub label: String,
    pub shaded: bool,
    pub color: GraphColor,
}

pub struct AxisDescription {
    pub min: f32,
    pub max: f32,
    pub unit: String,
}

pub struct LineChartProps {
    pub x_axis: AxisDescription,
    pub y_axis: AxisDescription,
}

#[derive(Clone)]
pub struct ViewBox {
    pub data_min: Point,
    pub data_max: Point,
}

impl ViewBox {
    pub(crate) fn data_point_to_view_box_point(&self, data_point: &Point) -> Point {
        let scale_x = 100.0 / (self.data_max.x - self.data_min.x);
        let scale_y = 100.0 / (self.data_max.y - self.data_min.y);

        Point {
            x: (data_point.x - self.data_min.x) * scale_x,
            y: (data_point.y - self.data_min.y) * scale_y,
        }
    }
}

pub fn line_chart(
    props: LineChartProps,
    datasets: impl SignalVecExt<Item = LineDataset> + 'static,
    mixin: impl FnMut(DomBuilder<SvgElement>) -> DomBuilder<SvgElement>,
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
    };

    svg!("svg", {
        .apply(mixin)
        .attr("width", "800px")
        .attr("height", "800px")
        .attr("viewBox", "-10 -10 120 120")
        .child(svg!("g", {
            .children_signal_vec(datasets.map(clone!(view_box => move |dataset| draw_data_set(dataset, view_box.clone()))))
        }))
        .child(layout_axis(vec![
            Axis {
                start_value: props.x_axis.min,
                end_value: props.x_axis.max,
                unit: props.x_axis.unit,
                ..Default::default()
            },
            Axis {
                start_value: props.y_axis.min,
                end_value: props.y_axis.max,
                unit: props.y_axis.unit,
                orientation: AxisOrientation::Vertical,
                ..Default::default()
            },
        ]))
    })
}

// move to shared code

fn line_points(points: &Vec<Point>, view_box: &ViewBox) -> String {
    points
        .iter()
        .map(|v| {
            let view_point = view_box.data_point_to_view_box_point(v);
            format!("{},{}", view_point.x, 100.0 - view_point.y)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn draw_data_set(dataset: LineDataset, view_box: ViewBox) -> Dom {
    let mut points = dataset.values;
    points.sort_by(|lhs, rhs| lhs.x.total_cmp(&rhs.x));

    if points.len() == 0 {
        return svg!("g");
    }

    let points_attr = line_points(&points, &view_box);

    svg!("g", {
        .apply(|builder| {

            if dataset.shaded {
                let left_x = view_box.data_point_to_view_box_point(&points[0]).x;
                let right_x = view_box.data_point_to_view_box_point(&points[points.len() - 1]).x;
                let points_attr = line_points(&points, &view_box);
                let points_attr = format!("{} {},100 {},100", points_attr, right_x, left_x);

                return builder.child(svg!("polygon", {
                    .attr("points", points_attr.as_str())
                    .attr("fill", dataset.color.to_css_stroke().as_str())
                    .attr("opacity", "30%")
                }))
            }

            builder
        })

        .child(svg!("polyline", {
            .attr("points", points_attr.as_str())
            .attr("fill", "none")
            .attr("stroke", dataset.color.to_css_stroke().as_str() )
        }))
    })
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
