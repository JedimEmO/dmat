#[cfg(test)]
mod test {
    use dominator_component_macros::component;
    use futures_signals::signal::always;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals::signal_vec::VecDiff;
    use std::num::ParseIntError;

    pub trait ToI32 {
        fn to_i32(self) -> Result<i32, ParseIntError>;
    }

    impl<T> ToI32 for T
    where
        T: ToString,
    {
        fn to_i32(self) -> Result<i32, ParseIntError> {
            i32::from_str_radix(self.to_string().as_str(), 10)
        }
    }

    #[macro_use]
    pub mod foo {
        use crate::test::ToI32;
        use dominator_component_macros::component;

        /// This is a trait that is used to define the props for the component
        /// # Example
        /// ```
        /// fn foo() -> () {}
        /// ```
        #[component(render_fn = some_button)]
        pub struct SomeButton<T: ToString + Default = i32, U: ToI32 + ToString + Default = i32> {
            /// The button label. This can be a signal, which allows us to update the label dynamically based on state changes
            #[signal]
            pub label: String,
            #[signal]
            pub foo: T,

            #[signal]
            pub bar: U,

            #[signal_vec]
            #[default(vec ! [123])]
            pub some_generic_signal_vec: i32,
        }

        pub fn some_button(props: impl SomeButtonPropsTrait) -> impl SomeButtonPropsTrait {
            props.take()
        }
    }

    use crate::test::foo::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn cmp_macro_test() {
        let rendered = some_button!({
            .foo_signal(always("test".to_string()))
        });

        let SomeButtonProps {
            some_generic_signal_vec,
            ..
        } = rendered.take();

        let mut copied = vec![];

        some_generic_signal_vec
            .for_each(|change| {
                match change {
                    VecDiff::Replace { values, .. } => {
                        copied = values;
                    }
                    _ => {}
                }
                async {}
            })
            .await;

        assert_eq!(copied, vec![123])
    }

    #[test]
    fn cmp_non_macro_test() {
        let rendered = some_button(SomeButtonProps::new().foo("hi there").bar(42));
        let SomeButtonProps { .. } = rendered.take();
    }

    #[test]
    fn attr_cmp_test() {
        let t = SomeButtonProps::new();

        let _t = t
            .foo_signal(always("test".to_string()))
            .foo(32)
            .bar(666)
            .label("hi".to_string())
            .label_signal(always("test".to_string()))
            .some_generic_signal_vec_signal_vec(futures_signals::signal_vec::always(vec![42, 666]))
            .apply(|dom_builder| dom_builder.attr("id", "yay"));
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn default_val_test() {
        #[component(render_fn = _r)]
        struct DefaultVal<T: ToI32 = i32> {
            #[signal]
            #[default(666)]
            foo: i32,

            #[default(123)]
            bar: T,

            #[signal_vec]
            #[default(vec![123, 666])]
            baz: i32,
        }

        async fn _r(p: impl DefaultValPropsTrait) {
            let DefaultValProps {
                foo: _,
                bar,
                baz,
                apply: _,
            } = p.take();
            assert_eq!(bar.to_i32().unwrap(), 123);

            let mut vec_val = vec![];

            baz.for_each(|change| {
                match change {
                    VecDiff::Replace { values, .. } => {
                        vec_val = values;
                    }
                    _ => {}
                }
                async {}
            })
            .await;

            assert_eq!(vec_val, vec![123, 666]);
        }

        default_val!({}).await;
    }
}
