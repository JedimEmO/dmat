pub trait EventSourced: Default {
    type Event;

    fn apply_event(&self, event: Self::Event);

    fn drain_events(&self, events: impl IntoIterator<Item = Self::Event>) {
        for event in events {
            self.apply_event(event);
        }
    }
}

pub enum MutableBTreeMapEvent<K, T: EventSourced> {
    Insert { key: K, value: T },
    Remove { key: K },
    Event { key: K, event: T::Event },
}
