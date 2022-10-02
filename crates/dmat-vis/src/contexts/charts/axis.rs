use crate::contexts::charts::line_chart::Point;
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

pub fn layout_axis(axis: Vec<Axis>) -> Dom {
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
        .map(|axis| draw_axis(axis))
        .collect::<Vec<Dom>>();

    let vertical = vertical
        .into_iter()
        .filter_map(|v| v)
        .map(|axis| draw_axis(axis))
        .collect::<Vec<Dom>>();

    svg!("g",{
        .children(horizontal)
        .children(vertical)
    })
}

pub fn draw_axis(axis: &Axis) -> Dom {
    match axis.orientation {
        AxisOrientation::Vertical => {
            svg!("g", {
                .child(draw_line(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 100.0 }))
                .child(svg!("text", {
                    .attr("x", "-10")
                    .attr("y", "100")
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.start_value).as_str())
                }))
                .child(svg!("text", {
                    .attr("x", "-10")
                    .attr("y", "0")
                    .attr("style", "font-size: 5px;")
                    .text(format!("{}", axis.end_value).as_str())
                }))
            })
        }
        AxisOrientation::Horizontal => {
            svg!("g", {
                .child(draw_line(Point { x: 0.0, y: 100.0 }, Point { x: 100.0, y: 100.0 }))
                .child(svg!("text", {
                    .attr("x", "0")
                    .attr("y", "105")
                    .class("dmat-axis-unit")
                    .text(format!("{}", axis.start_value).as_str())
                }))
                .child(svg!("text", {
                    .attr("x", "95")
                    .attr("y", "105")
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
        .attr("stroke-width", "0.5")
    })
}
