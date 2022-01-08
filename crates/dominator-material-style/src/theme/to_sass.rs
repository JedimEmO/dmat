pub trait ToSass {
    fn to_sass(&self) -> String;
}

#[derive(Clone)]
pub struct SassProperty<T: ToSass + Clone> {
    pub name: String,
    pub value: T,
}

impl<T: ToSass + Clone> ToSass for SassProperty<T> {
    fn to_sass(&self) -> String {
        format!("\"{}\": {}", self.name, self.value.to_sass())
    }
}

impl<TName, TProp: Clone + 'static> From<(TName, TProp)> for SassProperty<TProp>
where
    TName: Into<String>,
    TProp: ToSass,
{
    fn from(tuple: (TName, TProp)) -> Self {
        Self {
            name: tuple.0.into(),
            value: tuple.1,
        }
    }
}

pub fn render_sass_map(props: Vec<String>) -> String {
    format!("({})", props.join(",\n"))
}

#[cfg(test)]
mod test {
    use crate::theme::colors::Color;
    use crate::theme::to_sass::{SassProperty, ToSass};

    #[test]
    fn test_prop() {
        let color_prop: SassProperty<Color> = ("test", Color::Hex("".into())).into();
        let color_prop2 = SassProperty {
            name: "test".into(),
            value: Color::Hex("".into()),
        };

        let _out = color_prop.to_sass();
        let _out2 = color_prop2.to_sass();
    }
}
