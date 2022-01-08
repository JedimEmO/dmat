pub trait ToSass {
    fn to_sass(&self) -> String;
}

pub fn render_sass_property<N: AsRef<str>, T: ToSass>(name: N, prop: &T) -> String {
    format!("\"{}\": {}", name.as_ref(), prop.to_sass())
}

pub fn render_sass_map(props: Vec<String>) -> String {
    format!("({})", props.join(",\n"))
}
