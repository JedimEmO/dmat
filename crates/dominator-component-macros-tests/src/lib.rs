#[cfg(test)]
mod test {
    use dominator_component_macros::component;
    use futures_signals::signal::{Always, Signal};

    component! {
        name: TestCmp,
        render_fn: test_cmp,
        props: {
            <signal> label: String,
            click_handler<TClickHandler: Signal<Item=i32> = Always<i32>>: Option<TClickHandler>,
            disabled: Option<bool>,
        }
    }

    #[test]
    fn generated_component_test() {
        let _ = TestCmpProps::new();
    }
}