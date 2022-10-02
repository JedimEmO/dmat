use dmat_vis::contexts::charts::line_chart::{
    AxisDescription, GraphColor, LineChartProps, LineDataset, Point,
};
use dominator::Dom;
use futures_signals::signal_vec::MutableVec;

pub fn line_chart_demo() -> Dom {
    let datasets = MutableVec::new_with_values(vec![
        LineDataset {
            values: vec![
                Point { x: 0.0, y: 1000.0 },
                Point {
                    x: 1000.0,
                    y: 2500.0,
                },
                Point {
                    x: 4500.0,
                    y: 500.0,
                },
                Point {
                    x: 9000.0,
                    y: 10000.0,
                },
            ],
            label: "Test Set 1".to_string(),
            shaded: true,
            color: GraphColor::RGB {
                r: 128,
                g: 128,
                b: 0,
            },
        },
        LineDataset {
            values: vec![
                Point {
                    x: 3300.0,
                    y: 9000.0,
                },
                Point {
                    x: 8500.0,
                    y: 5000.0,
                },
            ],
            label: "Test Set 2".to_string(),
            shaded: false,
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
    };

    container!(|b| { b.child(line_chart!(props, datasets.signal_vec_cloned())) })
}
