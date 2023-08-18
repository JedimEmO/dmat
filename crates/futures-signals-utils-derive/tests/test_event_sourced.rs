#[cfg(test)]
mod test {
    use dominator_testing::async_yield;
    use futures_signals::signal::Mutable;
    use futures_signals::signal_map::MutableBTreeMap;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals::signal_vec::{MutableVec, VecDiff};
    use futures_signals_utils::event_sourced::{EventSourced, MutableBTreeMapEvent};
    use futures_signals_utils::Updateable;
    use futures_signals_utils_derive::EventSourced;

    #[derive(EventSourced, Default, Debug, Clone)]
    pub struct Inner {
        some_inner_value: Mutable<String>,
    }

    #[derive(EventSourced, Default)]
    pub struct Top {
        some_val: Mutable<String>,
        #[update_in_place_copied]
        some_vec: MutableVec<i32>,
        #[event_sourced]
        inner: Inner,
        #[event_sourced]
        inner_map: MutableBTreeMap<String, Inner>,
    }

    #[test]
    fn nested_event_sourced() {
        let top = Top::default();

        top.drain_events([
            TopEvent::UpdateInner(InnerEvent::Update(InnerEventUpdate {
                some_inner_value: Some(Mutable::new("testing inner".to_string())),
            })),
            TopEvent::UpdateInnerMap(MutableBTreeMapEvent::Insert {
                key: "hello".to_string(),
                value: Inner::default(),
            }),
            TopEvent::UpdateInnerMap(MutableBTreeMapEvent::Insert {
                key: "tmp".to_string(),
                value: Inner::default(),
            }),
            TopEvent::UpdateInnerMap(MutableBTreeMapEvent::Event {
                key: "hello".to_string(),
                event: InnerEvent::Update(InnerEventUpdate {
                    some_inner_value: Some(Mutable::new("testing inner 2".to_string())),
                }),
            }),
            TopEvent::UpdateInnerMap(MutableBTreeMapEvent::Remove {
                key: "tmp".to_string(),
            }),
        ]);

        assert_eq!(top.inner_map.lock_ref().len(), 1);
        assert!(top.inner_map.lock_ref().get("tmp").is_none());
        assert_eq!(
            top.inner_map
                .lock_ref()
                .get("hello")
                .unwrap()
                .some_inner_value
                .get_cloned(),
            "testing inner 2"
        );
    }

    #[test]
    fn test_derive_event_sourced() {
        let top = Top::default();

        assert_eq!(top.some_val.get_cloned(), "");

        top.apply_event(TopEvent::Update(TopEventUpdate {
            some_val: Some(Mutable::new("hello".to_string())),
        }));

        top.apply_event(TopEvent::UpdateSomeVec(VecDiff::Replace {
            values: vec![1, 2, 3],
        }));

        assert_eq!(top.some_val.get_cloned(), "hello");
        assert_eq!(top.some_vec.lock_ref().len(), 3);
        assert_eq!(top.some_vec.lock_ref()[2], 3);

        top.apply_event(TopEvent::UpdateSomeVec(VecDiff::Replace {
            values: vec![6],
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
            }),
            TopEvent::UpdateSomeVec(VecDiff::Replace {
                values: vec![1, 2, 3],
            }),
            TopEvent::UpdateSomeVec(VecDiff::Replace { values: vec![6] }),
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("world".to_string())),
            }),
        ];

        top.drain_events(events);

        assert_eq!(top.some_val.get_cloned(), "world");
        assert_eq!(top.get_some_vec_lock_ref().as_slice(), [6]);
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
            }),
            TopEvent::UpdateSomeVec(VecDiff::Replace {
                values: (0..3).collect(),
            }),
            TopEvent::Update(TopEventUpdate {
                some_val: Some(Mutable::new("world".to_string())),
            }),
        ]);

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 1);

        top.apply_event(TopEvent::UpdateSomeVec(VecDiff::Replace {
            values: (0..10).collect(),
        }));

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 2);

        top.apply_event(TopEvent::UpdateSomeVec(VecDiff::Replace {
            values: (0..10).collect(),
        }));

        async_yield().await;

        assert_eq!(update_count.get_cloned(), 3);
    }
}
