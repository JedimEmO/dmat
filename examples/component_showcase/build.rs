use dominator_material_style::render_dmat_scss;
use dominator_material_style::theme::colors::Colors;
use dominator_material_style::theme::dmat_theme::DmatTheme;
use std::fs;

fn main() {
    let theme = DmatTheme {
        colors: Colors {
            ..Default::default()
        },
        ..Default::default()
    };

    let scss_file_content = render_dmat_scss("example-app", theme);

    fs::write("style/dmat.generated.scss", scss_file_content.as_str()).unwrap();
}
