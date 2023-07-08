#[cfg(test)]
mod test {
    use dominator_component_macros::component;
    use futures_signals::signal::{Always, always, Signal, SignalExt};

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
            .disabled("hi".to_string())
            .disabled_signal(always(32))
            .label("yolo!".to_string())
            .apply(|dom_builder| dom_builder.attr("id", "yay"));

        let mapped_disabled = t.disabled.unwrap().map(|v| v + 1);
    }
}

