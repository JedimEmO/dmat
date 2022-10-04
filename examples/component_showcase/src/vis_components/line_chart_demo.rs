use dmat_vis::contexts::charts::line_chart::{
    AxisDescription, GraphColor, LineChartProps, LineDataset, Point,
};
use dominator::Dom;
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use std::future::Future;
use std::rc::Rc;
use std::time::Duration;
use wasm_timer::Delay;

pub fn line_chart_demo() -> Dom {
    let (data, changer_fut) = make_changing_source(10000.0, 20000.0, 50);
    let (data2, changer_fut2) = make_changing_source(10000.0, 20000.0, 10);

    let datasets = MutableVec::new_with_values(vec![
        LineDataset {
            values: Rc::new(move || Box::new(data.signal_vec().to_signal_cloned())),
            label: "Test Set 1".to_string(),
            shaded: false,
            color: GraphColor::RGB {
                r: 128,
                g: 128,
                b: 0,
            },
        },
        LineDataset {
            values: Rc::new(move || Box::new(data2.signal_vec().to_signal_cloned())),
            label: "Test Set 2".to_string(),
            shaded: true,
            color: GraphColor::RGB { r: 0, g: 0, b: 255 },
        },
    ]);

    let props = LineChartProps {
        x_axis: AxisDescription {
            min: 0.0,
            max: 10000.0,
            unit: "".to_string(),
        },
        y_axis: AxisDescription {
            min: 0.0,
            max: 20000.0,
            unit: "".to_string(),
        },
        width_px: 800,
        height_px: 400,
    };

    container!(|b| {
        b.child(line_chart!(props, datasets.signal_vec_cloned()))
            .future(changer_fut)
            .future(changer_fut2)
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
            Delay::new(Duration::from_millis(250)).await.unwrap();

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
