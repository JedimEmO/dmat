#[cfg(test)]
mod test {
    use dominator_testing::async_yield;
    use futures_signals::signal::Mutable;
    use futures_signals::signal_vec::MutableVec;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals_utils::prelude::*;
    use futures_signals_utils_derive::*;
    use std::default::Default;
    use wasm_bindgen_futures::spawn_local;

    #[derive(Updateable)]
    struct MyProp {
        pub param_a: Mutable<String>,
    }

    #[derive(Updateable)]
    struct MyStruct {
        pub param_a: Mutable<String>,
        pub some_vec: MutableVec<Mutable<String>>,
        pub prop: MyProp,

        #[skip]
        pub param_not_updated: Mutable<String>,

        // This will not call update on the elements, but will do a index-by-index compare and replace instead.
        // Useful if you don't want to hold an Updateable type, but can use partialeq and clone instead
        #[update_in_place_cloned]
        pub update_cloned: MutableVec<String>,

        #[update_in_place_copied]
        pub update_copied: MutableVec<i32>,
    }

    #[test]
    fn test_update() {
        let a = MyStruct {
            param_a: Default::default(),
            param_not_updated: Default::default(),
            some_vec: MutableVec::new_with_values(vec![
                Mutable::new("val1".to_string()),
                Mutable::new("val2".to_string()),
            ]),
            prop: MyProp {
                param_a: Default::default(),
            },
            update_cloned: Default::default(),
            update_copied: Default::default(),
        };

        let b = MyStruct {
            param_a: Mutable::new("42".to_string()),
            param_not_updated: Mutable::new("666".to_string()),
            some_vec: MutableVec::new_with_values(vec![
                Mutable::new("6".to_string()),
                Mutable::new("4".to_string()),
            ]),
            prop: MyProp {
                param_a: Mutable::new("new prop value".to_string()),
            },
            update_cloned: Default::default(),
            update_copied: Default::default(),
        };

        a.update(b);

        assert_eq!(a.param_a.get_cloned(), "42".to_string());
        assert_eq!(a.param_not_updated.get_cloned(), "".to_string());
        assert_eq!(a.some_vec.lock_ref().len(), 2);
        assert_eq!(a.some_vec.lock_ref()[0].get_cloned(), "6".to_string());
        assert_eq!(a.some_vec.lock_ref()[1].get_cloned(), "4".to_string());
        assert_eq!(a.prop.param_a.get_cloned(), "new prop value".to_string());
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn num_updates_verify() {
        let a = MyStruct {
            param_a: Default::default(),
            param_not_updated: Default::default(),
            some_vec: MutableVec::new_with_values(vec![Mutable::new("val1".to_string())]),
            prop: MyProp {
                param_a: Default::default(),
            },
            update_cloned: Default::default(),
            update_copied: Default::default(),
        };

        let update_count = Mutable::new(0);
        let update_count_ = update_count.clone();
        let update_count__ = update_count.clone();
        let update_count___ = update_count.clone();

        spawn_local(a.some_vec.signal_vec_cloned().for_each(move |_| {
            update_count_.replace_with(|v| *v + 1);
            async {}
        }));

        spawn_local(a.update_cloned.signal_vec_cloned().for_each(move |_| {
            update_count__.replace_with(|v| *v + 1);
            async {}
        }));

        spawn_local(a.update_copied.signal_vec().for_each(move |_| {
            update_count___.replace_with(|v| *v + 1);
            async {}
        }));

        for i in 0..100 {
            let b = MyStruct {
                param_a: Mutable::new(i.to_string()),
                param_not_updated: Mutable::new("666".to_string()),
                some_vec: MutableVec::new_with_values(vec![
                    Mutable::new("6".to_string()),
                    Mutable::new("4".to_string()),
                ]),
                prop: MyProp {
                    param_a: Default::default(),
                },
                update_cloned: MutableVec::new_with_values(vec![1.to_string(), 2.to_string()]),
                update_copied: MutableVec::new_with_values(vec![2]),
            };

            a.update(b);
        }

        async_yield().await;

        assert_eq!(update_count.get(), 5);

        for i in 0..100 {
            let b = MyStruct {
                param_a: Mutable::new(i.to_string()),
                param_not_updated: Mutable::new("666".to_string()),
                some_vec: MutableVec::new_with_values(vec![
                    Mutable::new("6".to_string()),
                    Mutable::new("4".to_string()),
                ]),
                prop: MyProp {
                    param_a: Default::default(),
                },
                update_cloned: MutableVec::new_with_values(vec![i.to_string(), i.to_string()]),
                update_copied: MutableVec::new_with_values(vec![i]),
            };

            a.update(b);
        }

        async_yield().await;

        assert_eq!(update_count.get(), 305);
    }
}
