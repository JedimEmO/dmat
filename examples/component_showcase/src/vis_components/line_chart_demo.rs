use dmat_components::components::layouts::*;
use dmat_components::components::*;
use dmat_vis::contexts::charts::axis::{AxisDescription, TickInfo};
use dmat_vis::contexts::charts::line_chart::{
    line_chart, DatasetValues, GraphColor, LineChartProps, LineDataset,
};
use dmat_vis::contexts::charts::point::Point;
use dominator::{Dom, DomBuilder};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use std::future::Future;
use std::rc::Rc;
use std::time::Duration;
use wasm_timer::Delay;
use web_sys::SvgElement;

pub fn line_chart_demo() -> Dom {
    container!({ .children([dynamic_chart(), static_centered_axis_chart()]) })
}

fn static_centered_axis_chart() -> Dom {
    let datasets = MutableVec::new_with_values(vec![
        LineDataset {
            values: DatasetValues::Static(vec![
                Point { x: -10.0, y: -10.0 },
                Point { x: -5.0, y: 2.0 },
                Point { x: 5.0, y: 13.0 },
                Point { x: 10.0, y: 5.0 },
            ]),
            label: "Test Set 1".to_string(),
            shaded: false,
            color: GraphColor::RGB {
                r: 0,
                g: 118,
                b: 208,
            },
        },
        LineDataset {
            values: DatasetValues::Static(vec![
                Point { x: -20.0, y: 10.0 },
                Point { x: -15.0, y: 2.0 },
                Point { x: 5.0, y: -5.0 },
                Point { x: 10.0, y: -10.0 },
            ]),
            label: "Test Set 1".to_string(),
            shaded: true,
            color: GraphColor::RGB {
                r: 255,
                g: 146,
                b: 0,
            },
        },
    ]);

    let props = LineChartProps {
        x_axis: AxisDescription {
            min: -20.0,
            max: 10.0,
            ticks: Some(TickInfo {
                count: 5,
                format: |v| format!("{:.0}", v),
            }),
        },
        y_axis: AxisDescription {
            min: -10.0,
            max: 20.0,
            ticks: Some(TickInfo {
                count: 10,
                format: |v| format!("{:.0}", v),
            }),
        },
        width_px: 200,
        height_px: 200,
    };

    card!({
        .child(content_block!(ContentBlockProps {
        title_section: Some(title!({
            .header_text("Static data line chart".to_string())
            .sub_header_text(Some("Axis center within view box".to_string()))
        })),
        media_section: Some(line_chart!(props, datasets.signal_vec_cloned())),
        ..Default::default()
    }))
    })
}

fn dynamic_chart() -> Dom {
    let (data, changer_fut) = make_changing_source(10000.0, 5000.0, 50);
    let (data2, changer_fut2) = make_changing_source(10000.0, 20000.0, 10);

    let datasets = MutableVec::new_with_values(vec![
        LineDataset {
            values: DatasetValues::Signal(Rc::new(move || {
                Box::new(data.signal_vec().to_signal_cloned())
            })),
            label: "Test Set 1".to_string(),
            shaded: false,
            color: GraphColor::RGB {
                r: 0,
                g: 118,
                b: 208,
            },
        },
        LineDataset {
            values: DatasetValues::Signal(Rc::new(move || {
                Box::new(data2.signal_vec().to_signal_cloned())
            })),
            label: "Test Set 2".to_string(),
            shaded: true,
            color: GraphColor::RGB { r: 254, g: 0, b: 6 },
        },
    ]);

    let props = LineChartProps {
        x_axis: AxisDescription {
            min: 0.0,
            max: 10000.0,
            ticks: Some(TickInfo {
                count: 5,
                format: |v| format!("{:.0}", v),
            }),
        },
        y_axis: AxisDescription {
            min: 0.0,
            max: 20000.0,
            ticks: Some(TickInfo {
                count: 20,
                format: |v| format!("{:.0}k", (v / 1000.0)),
            }),
        },
        width_px: 200,
        height_px: 200,
    };

    card!({
        .child(content_block!(ContentBlockProps {
        title_section: Some(title!({
            .header_text("Dynamic data line chart".to_string())
        })),
        media_section: Some(line_chart(
            props,
            datasets.signal_vec_cloned(),
            |b: DomBuilder<SvgElement>| b.future(changer_fut).future(changer_fut2),
        )),
        ..Default::default()
    }))
    })
}

fn make_changing_source(
    range_x: f32,
    range: f32,
    count: usize,
) -> (MutableVec<Point>, impl Future<Output = ()>) {
    let out = MutableVec::new_with_values(
        (0..count)
            .map(|v| Point {
                x: (v as f32) * range_x / (count as f32),
                y: range * rand::random::<f32>(),
            })
            .collect(),
    );

    let out_clone = out.clone();

    let changer_fut = async move {
        loop {
            Delay::new(Duration::from_millis(500)).await.unwrap();

            {
                let mut cpy = out_clone.lock_ref().to_vec();

                for v in 0..cpy.len() {
                    let w = cpy.get_mut(v).unwrap();
                    w.y = range * rand::random::<f32>();
                }

                out_clone.lock_mut().replace(cpy);
            }
        }
    };

    (out, changer_fut)
}
