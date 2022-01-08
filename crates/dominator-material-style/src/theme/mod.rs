pub mod breakpoints;
pub mod colors;
pub mod components;
pub mod dmat_theme;
pub mod layout;
pub mod to_sass;

#[cfg(test)]
mod test {
    use crate::{render_theme, DmatTheme};
    use grass::Options;

    #[test]
    fn verify_rendered_sass() {
        let theme = DmatTheme::default();
        let theme_file_content = render_theme("test-theme", theme);

        println!("{}", theme_file_content);

        grass::from_string(theme_file_content, &Options::default()).unwrap();
    }
}