use crate::contexts::charts::point::Point;
use crate::contexts::charts::view_box::ViewBox;
use dominator::Dom;
use itertools_num::linspace;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AxisOrientation {
    Horizontal,
    Vertical,
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

pub fn layout_axis(x_axis: &AxisDescription, y_axis: &AxisDescription, view_box: &ViewBox) -> Dom {
    svg!("g",{
        .class("dmat-axis-group")
        .child(draw_axis(x_axis, AxisOrientation::Horizontal, view_box))
        .child(draw_axis(y_axis, AxisOrientation::Vertical, view_box))
    })
}

pub fn draw_axis(axis: &AxisDescription, orientation: AxisOrientation, view_box: &ViewBox) -> Dom {
    let mut zero_point = view_box.data_point_to_view_box_point(&Point { x: 0.0, y: 0.0 });

    if zero_point.x < 0.0 || zero_point.x > view_box.view_width {
        zero_point.x = 0.0;
    }

    if zero_point.y < 0.0 || zero_point.y > view_box.view_height {
        zero_point.y = view_box.view_height;
    }

    match orientation {
        AxisOrientation::Vertical => {
            svg!("g", {
                .class("dmat-axis")
                .child(draw_line(Point { x: zero_point.x, y: 0.0 }, Point { x: zero_point.x, y: view_box.view_height }))
                .apply(|b| {
                    if let Some(t) = &axis.ticks {
                        return b.child(svg!("g", {
                            .class("dmat-axis-ticks")
                            .children(linspace(axis.min, axis.max, t.count)
                                .map(|tick_point| {
                                    let txt = (t.format)(tick_point);
                                    let point = Point { x: 0.0, y: tick_point};
                                    let point = view_box.data_point_to_view_box_point(&point);
                                    let y = format!("{}", point.y);
                                    svg!("g", {
                                        .child(svg!("text", {
                                            .attr("x", format!("{}", zero_point.x - 20.0).as_str())
                                            .attr("y", y.as_str())
                                            .class("dmat-axis-unit")
                                            .text(txt.as_str())
                                        }))
                                        .child(svg!("line", {
                                            .attr("x1", format!("{}", zero_point.x - 1.0).as_str())
                                            .attr("y1", y.as_str())
                                            .attr("x2", format!("{}", zero_point.x + 5.0).as_str())
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
                .child(draw_line(Point { x: 0.0, y: zero_point.y }, Point { x: view_box.view_width, y: zero_point.y}))
                .apply(|b| {
                    if let Some(t) = &axis.ticks {
                        return b.child(svg!("g", {
                            .class("dmat-axis-ticks")
                            .children(linspace(axis.min, axis.max, t.count)
                                .map(|tick_point| {
                                    let txt = (t.format)(tick_point);
                                    let point = Point { x: tick_point, y: 0.0};
                                    let point = view_box.data_point_to_view_box_point(&point);
                                    let x = format!("{}", point.x);
                                    let text_x = format!("{}", point.x - txt.len() as f32 * 5.0/2.0);
                                    let y = format!("{}", point.y + 10.0);
                                    let y2 = format!("{}", point.y);
                                    let y3 = format!("{}", point.y - 5.0);

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
