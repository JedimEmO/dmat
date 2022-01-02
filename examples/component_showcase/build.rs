use dominator_material_style::render_dmat_scss;
use dominator_material_style::theme::{Color, Colors, DmatTheme};
use std::path::Path;
use std::{env, fs};

fn main() {
    let theme = DmatTheme {
        colors: Colors {
            ..Default::default()
        },
    };

    let scss_file_content = render_dmat_scss("example-app", theme);

    fs::write("style/dmat.generated.scss", scss_file_content.as_str()).unwrap();
}
