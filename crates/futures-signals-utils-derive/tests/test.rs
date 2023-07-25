#[cfg(test)]
mod test {
    use futures_signals::signal::Mutable;
    use futures_signals::signal_vec::MutableVec;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals_utils::Updateable;
    use futures_signals_utils_derive::*;
    use std::default::Default;
    use std::time::Duration;

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
        };

        a.update(&b);

        assert_eq!(a.param_a.get_cloned(), "42".to_string());
        assert_eq!(a.param_not_updated.get_cloned(), "".to_string());
        assert_eq!(a.some_vec.lock_ref().len(), 2);
        assert_eq!(a.some_vec.lock_ref()[0].get_cloned(), "6".to_string());
        assert_eq!(a.some_vec.lock_ref()[1].get_cloned(), "4".to_string());
        assert_eq!(a.prop.param_a.get_cloned(), "new prop value".to_string());
    }

    #[tokio::test]
    async fn num_updates_verify() {
        let a = MyStruct {
            param_a: Default::default(),
            param_not_updated: Default::default(),
            some_vec: MutableVec::new_with_values(vec![Mutable::new("val1".to_string())]),
            prop: MyProp {
                param_a: Default::default(),
            },
        };

        let update_count = Mutable::new(0);
        let update_count_ = update_count.clone();

        tokio::spawn(a.some_vec.signal_vec_cloned().for_each(move |_| {
            update_count_.replace_with(|v| *v + 1);
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
            };

            a.update(&b);
        }

        tokio::time::sleep(Duration::from_millis(100)).await;

        assert_eq!(update_count.get_cloned(), 2);
    }
}
