#[cfg(test)]
mod test {
    use dominator_testing::async_yield;
    use futures_signals::signal::Mutable;
    use futures_signals::signal_vec::MutableVec;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals_utils::event_sourced::EventSourced;
    use futures_signals_utils::Updateable;
    use futures_signals_utils_derive::EventSourced;

    #[derive(EventSourced, Default)]
    struct Inner {}

    #[derive(EventSourced, Default)]
    struct Top {
        some_val: Mutable<String>,
        #[update_in_place_copied]
        some_vec: MutableVec<i32>,
        #[event_sourced]
        inner: Inner,
    }

    #[test]
    fn test_derive_event_sourced() {
        let top = Top::default();

        assert_eq!(top.some_val.get_cloned(), "");

        top.apply_event(TopEvent::Update(TopEventUpdate {
            some_val: Some(Mutable::new("hello".to_string())),
            some_vec: Some(MutableVec::new_with_values(vec![1, 2, 3])),
        }));

        assert_eq!(top.some_val.get_cloned(), "hello");
        assert_eq!(top.some_vec.lock_ref().len(), 3);
        assert_eq!(top.some_vec.lock_ref()[2], 3);

        top.apply_event(TopEvent::Update(TopEventUpdate {
            some_vec: Some(MutableVec::new_with_values(vec![6])),
            ..Default::default()
        }));

        assert_eq!(top.some_val.get_cloned(), "hello");
        assert_eq!(top.some_vec.lock_ref().len(), 1);
        assert_eq!(top.some_vec.lock_ref()[0], 6);

        assert_eq!(top.get_some_val(), "hello");
    }

    #[test]
    fn test_event_sourced_drain_events() {
        let top = Top::default();

        let events = vec![
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("hello".to_string())),
                some_vec: Some(MutableVec::new_with_values(vec![1, 2, 3])),
            }),
            TopEvent::Update(TopEventUpdate {
                some_vec: Some(MutableVec::new_with_values(vec![6])),
                ..Default::default()
            }),
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("world".to_string())),
                ..Default::default()
            }),
        ];

        top.drain_events(events);

        assert_eq!(top.some_val.get_cloned(), "world");
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_event_sourced_signals() {
        let top = Top::default();

        let update_count = Mutable::new(0);
        let update_count_ = update_count.clone();

        wasm_bindgen_futures::spawn_local(top.some_vec_signal_vec().for_each(move |_| {
            update_count_.replace_with(|v| *v + 1);
            async {}
        }));

        top.drain_events(vec![
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("hello".to_string())),
                some_vec: Some(MutableVec::new_with_values(vec![1, 2, 3])),
            }),
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("world".to_string())),
                ..Default::default()
            }),
        ]);

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 3);

        top.apply_event(TopEvent::Update(TopEventUpdate {
            some_vec: Some(MutableVec::new_with_values((0..=10).collect())),
            ..Default::default()
        }));

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 14);

        top.apply_event(TopEvent::Update(TopEventUpdate {
            some_vec: Some(MutableVec::new_with_values((0..=10).collect())),
            ..Default::default()
        }));

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 14);
    }
}
