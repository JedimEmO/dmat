use crate::contexts::charts::line_chart::{AxisDescription, Point, ViewBox};
use dominator::traits::AsStr;
use dominator::Dom;
use itertools_num::linspace;

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

#[derive(PartialEq, Eq, Copy, Clone)]
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

pub fn layout_axis(x_axis: &AxisDescription, y_axis: &AxisDescription, view_box: &ViewBox) -> Dom {
    svg!("g",{
        .child(draw_axis(x_axis, AxisOrientation::Horizontal, view_box))
        .child(draw_axis(y_axis, AxisOrientation::Vertical, view_box))
    })
}

pub fn draw_axis(axis: &AxisDescription, orientation: AxisOrientation, view_box: &ViewBox) -> Dom {
    match orientation {
        AxisOrientation::Vertical => {
            svg!("g", {
                .class("dmat-axis")
                .child(draw_line(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: view_box.view_height }))
                .apply(|b| {
                    if let Some(t) = &axis.ticks {
                        return b.child(svg!("g", {
                            .class("dmat-axis-ticks")
                            .children(linspace(axis.min, axis.max, t.count)
                                .map(|tick_point| {
                                    let txt = format!("{}", (t.format)(tick_point));
                                    let point = Point { x: 0.0, y: tick_point};
                                    let point = view_box.data_point_to_view_box_point(&point);
                                    let y = format!("{}", view_box.view_height - point.y);
                                    svg!("g", {
                                        .child(svg!("text", {
                                            .attr("x", "-20")
                                            .attr("y", y.as_str())
                                            .class("dmat-axis-unit")
                                            .text(txt.as_str())
                                        }))
                                        .child(svg!("line", {
                                            .attr("x1", "-1")
                                            .attr("y1", y.as_str())
                                            .attr("x2", "5")
                                            .attr("y2", y.as_str())
                                            .attr("stroke", "black")
                                        }))
                                    })

                            }))
                        }))
                    }

                    b
                })
            })
        }
        AxisOrientation::Horizontal => {
            svg!("g", {
                .child(draw_line(Point { x: 0.0, y: view_box.view_height }, Point { x: view_box.view_width, y: view_box.view_height }))
                .apply(|b| {
                    if let Some(t) = &axis.ticks {
                        return b.child(svg!("g", {
                            .class("dmat-axis-ticks")
                            .children(linspace(axis.min, axis.max, t.count)
                                .map(|tick_point| {
                                    let txt = format!("{}", (t.format)(tick_point));
                                    let point = Point { x: tick_point, y: 0.0};
                                    let point = view_box.data_point_to_view_box_point(&point);
                                    let x = format!("{}", point.x);
                                    let text_x = format!("{}", point.x - txt.len() as f32 * 5.0/2.0);
                                    let y = format!("{}", view_box.view_height + 10.0);
                                    let y2 = format!("{}", view_box.view_height);
                                    let y3 = format!("{}", view_box.view_height - 5.0);
                                    svg!("g", {
                                        .child(svg!("text", {
                                            .attr("x", text_x.as_str())
                                            .attr("y", y.as_str())
                                            .class("dmat-axis-unit")
                                            .text(txt.as_str())
                                        }))
                                        .child(svg!("line", {
                                            .attr("x1", x.as_str())
                                            .attr("y1", y2.as_str())
                                            .attr("x2", x.as_str())
                                            .attr("y2", y3.as_str())
                                            .attr("stroke", "black")
                                        }))
                                    })

                            }))
                        }))
                    }

                    b
                })
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
