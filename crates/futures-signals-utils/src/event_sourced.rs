use serde::{Deserialize, Serialize};

pub trait EventSourced: Default {
    type Event;

    fn apply_event(&self, event: Self::Event);

    fn drain_events(&self, events: impl IntoIterator<Item = Self::Event>) {
        for event in events {
            self.apply_event(event);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MutableBTreeMapEvent<K, T: EventSourced> {
    Insert { key: K, value: T },
    Remove { key: K },
    Event { key: K, event: T::Event },
}

#[derive(Serialize, Deserialize)]
pub enum MutableVecEvent<T: EventSourced + Clone> {
    Insert { index: usize, value: T },
    Remove { index: usize },
    Event { index: usize, event: T::Event },
    Swap { index: usize, other: usize },
    Clear,
    Replace { values: Vec<T> },
}
