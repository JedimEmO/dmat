use crate::theme::dmat_theme::{render_theme, DmatTheme};
use rust_embed::RustEmbed;

pub mod theme;

#[derive(RustEmbed)]
#[folder = "src/style"]
pub(crate) struct DmatScssAssets;

pub fn render_dmat_scss(theme_name: &str, theme: DmatTheme) -> String {
    let content = get_concatenated_scss();
    let theme = render_theme(theme_name, theme);

    format!(
        r"// Auto generated DMAT scss file - please consult your build.rs file for sources 
{}

{}
@include dmat(${});",
        theme, content, theme_name
    )
}

pub(crate) fn get_concatenated_scss() -> String {
    DmatScssAssets::iter()
        .map(|path| DmatScssAssets::get(&path).unwrap())
        .map(|file| String::from_utf8(file.data.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .split('\n')
        .filter(|v| !v.contains("@import"))
        .collect::<Vec<&str>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use crate::{render_dmat_scss, DmatTheme};
    use grass::Options;

    #[test]
    fn verify_theme_compilation() {
        let files = render_dmat_scss("test-theme", DmatTheme::default());

        grass::from_string(files, &Options::default()).unwrap();
    }
}
