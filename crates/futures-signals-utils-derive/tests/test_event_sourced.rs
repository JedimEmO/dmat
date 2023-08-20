#[cfg(test)]
mod test {
    use dominator_testing::async_yield;
    use futures_signals::signal::Mutable;
    use futures_signals::signal_map::MutableBTreeMap;
    use futures_signals::signal_vec::SignalVecExt;
    use futures_signals::signal_vec::{MutableVec, VecDiff};
    use futures_signals_utils::event_sourced::{EventSourced, MutableBTreeMapEvent};
    use futures_signals_utils::prelude::{EventStore, MutableVecEvent};
    use futures_signals_utils::updateable::Updateable;
    use futures_signals_utils_derive::EventSourced;
    use serde::{Deserialize, Serialize};
    use std::time::UNIX_EPOCH;

    #[derive(EventSourced, Default, Debug, Clone, Serialize, Deserialize)]
    pub struct Inner {
        some_inner_value: Mutable<String>,
    }

    #[derive(EventSourced, Default, Debug, Serialize, Deserialize, Clone)]
    pub struct Top {
        some_val: Mutable<String>,
        #[update_in_place_copied]
        some_vec: MutableVec<i32>,
        #[event_sourced]
        inner: Inner,
        #[event_sourced]
        inner_map: MutableBTreeMap<String, Inner>,
        #[event_sourced]
        inner_vec: MutableVec<Inner>,
    }

    #[test]
    fn test_event_store() {
        let mut top_store: EventStore<Top> = EventStore::new();

        let events = vec![
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
            // Make two inners in the inner vec, delete one, and update the other
            TopEvent::UpdateInnerVec(MutableVecEvent::Replace {
                values: vec![Inner::default(), Inner::default()],
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Remove { index: 0 }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Event {
                index: 0,
                event: InnerEvent::Update(InnerEventUpdate {
                    some_inner_value: Some(Mutable::new("testing inner 3".to_string())),
                }),
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Insert {
                index: 0,
                value: Inner::default(),
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Event {
                index: 0,
                event: InnerEvent::Update(InnerEventUpdate {
                    some_inner_value: Some(Mutable::new("testing inner 4".to_string())),
                }),
            }),
        ];

        for event in events.into_iter() {
            top_store.push_event(
                event,
                std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
        }

        assert_eq!(
            top_store
                .current_value
                .lock_ref()
                .inner_vec
                .lock_ref()
                .len(),
            2
        );

        top_store.undo(2);

        assert_eq!(
            top_store
                .current_value
                .lock_ref()
                .inner_vec
                .lock_ref()
                .len(),
            1
        );

        println!("{:?}", top_store.current_value.lock_ref());
        assert_eq!(
            top_store
                .current_value
                .lock_ref()
                .inner_vec
                .lock_ref()
                .get(0)
                .unwrap()
                .some_inner_value
                .lock_ref()
                .clone(),
            "testing inner 3".to_string()
        );
    }

    #[test]
    fn nested_event_sourced() {
        let top = Top::default();

        let events = [
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
            // Make two inners in the inner vec, delete one, and update the other
            TopEvent::UpdateInnerVec(MutableVecEvent::Replace {
                values: vec![Inner::default(), Inner::default()],
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Remove { index: 0 }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Event {
                index: 0,
                event: InnerEvent::Update(InnerEventUpdate {
                    some_inner_value: Some(Mutable::new("testing inner 3".to_string())),
                }),
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Insert {
                index: 0,
                value: Inner::default(),
            }),
            TopEvent::UpdateInnerVec(MutableVecEvent::Event {
                index: 0,
                event: InnerEvent::Update(InnerEventUpdate {
                    some_inner_value: Some(Mutable::new("testing inner 4".to_string())),
                }),
            }),
        ];

        let event_log_json = serde_json::to_string(&events).unwrap();
        let events: Vec<TopEvent> = serde_json::from_str(&event_log_json).unwrap();

        top.drain_events(events);

        assert_eq!(top.inner_map.lock_ref().len(), 1);
        assert!(top.inner_map_lock_ref().get("tmp").is_none());
        assert_eq!(
            top.inner_map
                .lock_ref()
                .get("hello")
                .unwrap()
                .some_inner_value
                .get_cloned(),
            "testing inner 2"
        );

        assert_eq!(
            top.inner_vec
                .lock_ref()
                .get(0)
                .unwrap()
                .some_inner_value
                .get_cloned(),
            "testing inner 4"
        );

        top.apply_event(TopEvent::UpdateInnerVec(MutableVecEvent::Swap {
            index: 0,
            other: 1,
        }));

        assert_eq!(
            top.inner_vec_lock_ref()[0].some_inner_value.get_cloned(),
            "testing inner 3"
        );

        top.apply_event(TopEvent::UpdateInnerVec(MutableVecEvent::Clear));

        assert_eq!(top.inner_vec_lock_ref().len(), 0);
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

        assert_eq!(top.some_val_cloned(), "hello");
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
        assert_eq!(top.some_vec_lock_ref().as_slice(), [6]);
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
