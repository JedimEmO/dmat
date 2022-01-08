use std::fs;

use dominator_material_style::render_dmat_scss;
use dominator_material_style::theme::breakpoints::BreakpointValue;
use dominator_material_style::theme::dmat_theme::DmatTheme;
use dominator_material_style::theme::layout::{DisplayUnit, Layout};

fn main() {
    let theme = DmatTheme {
        layout: Layout {
            margin: BreakpointValue {
                large: DisplayUnit::Pixels(32),
                ..Default::default()
            },
        },
        ..Default::default()
    };

    let scss_file_content = render_dmat_scss("example-app", theme);

    fs::write("style/dmat.generated.scss", scss_file_content.as_str()).unwrap();
}
