#[cfg(test)]
mod test {
    use futures_signals::signal::always;

    #[macro_use]
    pub mod foo {
        use dominator_component_macros::component;

        #[component(render_fn = some_button)]
        pub struct SomeButton<T: Default = i32, U: Default = String> {
            #[signal]
            pub label: String,
            #[signal]
            pub foo: T,

            #[signal_vec]
            pub bar: String,

            #[signal_vec]
            pub some_generic_signal_vec: U,
        }

        pub fn some_button(props: impl SomeButtonPropsTrait) -> i32 {
            let _props = props.take();

            42
        }
    }

    use crate::test::foo::*;

    #[test]
    fn cmp_macro_test() {
        let rendered = some_button!({
            .foo_signal(always("test".to_string()))
        });

        assert_eq!(rendered, 42);
    }

    #[test]
    fn cmp_non_macro_test() {
        let rendered = some_button(SomeButtonProps::new().foo("hi there"));
        assert_eq!(rendered, 42);
    }

    #[test]
    fn attr_cmp_test() {
        let t = SomeButtonProps::new();

        let _t = t
            .foo_signal(always("test".to_string()))
            .foo(32)
            .bar(vec!["hellothere".to_string()])
            .label("hi".to_string())
            .label_signal(always("test".to_string()))
            .some_generic_signal_vec_signal(futures_signals::signal_vec::always(vec![42, 666]))
            .apply(|dom_builder| dom_builder.attr("id", "yay"));
    }
}
