use crate::contexts::charts::line_chart::{Point, ViewBox};
use dominator::Dom;

pub enum AxisPosition {
    /// Primary position, meaning left or bottom depending on the axis orientation
    Primary,
    /// Secondary position, meaning right or top depending on the orientation
    Secondary,
}

impl Default for AxisPosition {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum AxisOrientation {
    Horizontal,
    Vertical,
}

impl Default for AxisOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

#[derive(Default)]
pub struct Axis {
    pub unit: String,
    pub start_value: f32,
    pub end_value: f32,
    pub position: AxisPosition,
    pub orientation: AxisOrientation,
}

pub fn layout_axis(axis: Vec<Axis>, view_box: &ViewBox) -> Dom {
    let (horizontal, vertical): (Vec<_>, Vec<_>) = axis
        .iter()
        .map(|v| match v.orientation {
            AxisOrientation::Horizontal => (Some(v), None),
            _ => (None, Some(v)),
        })
        .unzip();

    let horizontal = horizontal
        .into_iter()
        .filter_map(|v| v)
        .map(|axis| draw_axis(axis, view_box))
        .collect::<Vec<Dom>>();

    let vertical = vertical
        .into_iter()
        .filter_map(|v| v)
        .map(|axis| draw_axis(axis, view_box))
        .collect::<Vec<Dom>>();

    svg!("g",{
        .children(horizontal)
        .children(vertical)
    })
}

pub fn draw_axis(axis: &Axis, view_box: &ViewBox) -> Dom {
    match axis.orientation {
        AxisOrientation::Vertical => {
            svg!("g", {
                .child(draw_line(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: view_box.view_height }))
                .child(svg!("text", {
                    .attr("x", "-20")
                    .attr("y", format!("{}", view_box.view_height).as_str())
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.start_value).as_str())
                }))
                .child(svg!("text", {
                    .attr("x", "-20")
                    .attr("y", "0")
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.end_value).as_str())
                }))
            })
        }
        AxisOrientation::Horizontal => {
            svg!("g", {
                .child(draw_line(Point { x: 0.0, y: view_box.view_height }, Point { x: view_box.view_width, y: view_box.view_height }))
                .child(svg!("text", {
                    .attr("x", "0")
                    .attr("y", format!("{}", view_box.view_height + 15.0 ).as_str())
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.start_value).as_str())
                }))
                .child(svg!("text", {
                    .attr("x", format!("{}", view_box.view_width - format!("{}", axis.start_value).len()as f32*15.0/2.0).as_str())
                    .attr("y", format!("{}", view_box.view_height + 15.0).as_str())
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.end_value).as_str())
                }))
            })
        }
    }
}

fn draw_line(start: Point, end: Point) -> Dom {
    svg!("line", {
        .attr("x1", start.x.to_string().as_str())
        .attr("x2", end.x.to_string().as_str())
        .attr("y1", start.y.to_string().as_str())
        .attr("y2", end.y.to_string().as_str())
        .attr("stroke", "black")
        .attr("stroke-width", "1")
    })
}
