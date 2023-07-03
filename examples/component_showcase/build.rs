use dmat_components_style::render_dmat_scss;
use dmat_components_style::theme::breakpoints::BreakpointValue;
use dmat_components_style::theme::dmat_theme::DmatTheme;
use dmat_components_style::theme::layout::{DisplayUnit, Layout};


fn main() {
    let theme = DmatTheme {
        layout: Layout {
            margin: BreakpointValue {
                small: DisplayUnit::Pixels(8),
                medium: DisplayUnit::Pixels(16),
                large: DisplayUnit::Pixels(32),
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let _scss_file_content = render_dmat_scss("example-app", theme);

    // fs::write("style/dmat.generated.scss", scss_file_content.as_str()).unwrap();
}
