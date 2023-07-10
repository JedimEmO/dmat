#[cfg(test)]
mod test {
    use dominator_component_macros::component;
    use dominator_component_macros::component_attr;
    use futures_signals::signal::{ always, SignalExt};

    component! {
        name: TestCmp,
        render_fn: test_cmp,
        props: {
            #[signal]
            label: String,
            click_handler<TClickHandler: FnMut() -> () = fn () ->()>: TClickHandler,
            #[signal]
            disabled<T: Default = i32>: T,
        }
    }

    #[test]
    fn generated_component_test() {
        let t = TestCmpProps::new();

        let f  = 32;

        let t = t.label_signal(always("test".to_string()))
            .click_handler(||{ println!("clicked{}!", f) })
            .disabled_signal(always(32))
            .disabled("hi".to_string())
            .label("yolo!".to_string())
            .apply(|dom_builder| dom_builder.attr("id", "yay"));

        let _mapped_disabled = t.disabled.unwrap().map(|v| format!("{} + 1", v));
    }

    #[component_attr(render_fn = test_cmp)]
    struct SomeButton<T: Default = i32> {
        #[signal]
        pub label: String,
        #[signal]
        pub foo: T
    }

    fn some_button(mut props: impl SomeButtonPropsTrait) -> () {
        let _foo = props.foo();
    }

    #[test]
    fn attr_cmp_test() {
        let t = SomeButtonProps::new();

        let _t = t.foo_signal(always("test".to_string()))
            .foo(32)
            .label("hi".to_string())
            .label_signal(always("test".to_string()))
            .apply(|dom_builder| dom_builder.attr("id", "yay"));
    }

}
