use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait EventSourced: Default {
    type Event: Serialize + for<'a> Deserialize<'a> + Clone + Debug;

    fn apply_event(&self, event: Self::Event);

    fn drain_events(&self, events: impl IntoIterator<Item = Self::Event>) {
        for event in events {
            self.apply_event(event);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Event<TEvent> {
    pub sequence_number: u64,
    pub timestamp: u64,
    pub data: TEvent,
}

#[derive(Serialize, Deserialize)]
pub struct EventStore<T: EventSourced> {
    pub current_value: Mutable<T>,
    next_sequence_number: u64,
    events: Vec<Event<T::Event>>,
    undone_events: Vec<Event<T::Event>>,
    snapshots: Vec<(u64, T)>,
}

impl<T: EventSourced + Clone + Serialize + for<'a> Deserialize<'a>> EventStore<T> {
    pub fn new() -> Self {
        Self {
            current_value: Mutable::new(T::default()),
            next_sequence_number: 1,
            events: Vec::new(),
            undone_events: Vec::new(),
            snapshots: vec![(0, T::default())],
        }
    }
    pub fn undo(&mut self, count: usize) {
        let mut undone_events = Vec::new();

        for _ in 0..count {
            if let Some(event) = self.events.pop() {
                undone_events.push(event);
            }
        }

        if undone_events.is_empty() {
            return;
        }

        self.undone_events.extend(undone_events);

        let head_sequence_number = self.events.last().map(|e| e.sequence_number).unwrap_or(0);
        self.next_sequence_number = head_sequence_number + 1;
        let (snapshot_sequence_number, snapshot) = self.get_snapshot(head_sequence_number);

        let events_to_replay = self
            .events
            .iter()
            .filter(|e| e.sequence_number > snapshot_sequence_number);
        snapshot.drain_events(events_to_replay.map(|e| e.data.clone()));

        self.current_value.set(snapshot);
    }

    pub fn redo(&mut self, _count: usize) {}

    pub fn push_event(&mut self, event_data: T::Event, timestamp: u64) {
        self.undone_events.clear();
        let seq = self.next_sequence_number;
        self.next_sequence_number += 1;

        let event = Event {
            sequence_number: seq,
            timestamp,
            data: event_data,
        };

        self.current_value
            .lock_ref()
            .apply_event(event.data.clone());
        self.events.push(event);
    }

    fn get_snapshot(&self, sequence_number: u64) -> (u64, T) {
        // Unwrapping is safe because we always have at least one snapshot
        self.snapshots
            .iter()
            .rev()
            .find(|(seq, _)| *seq <= sequence_number)
            .cloned()
            .unwrap()
    }
}
impl<T: EventSourced + Clone + Serialize + for<'a> Deserialize<'a>> Default for EventStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MutableBTreeMapEvent<K, T: EventSourced> {
    Insert { key: K, value: T },
    Remove { key: K },
    Event { key: K, event: T::Event },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MutableVecEvent<T: EventSourced + Clone> {
    Insert { index: usize, value: T },
    Remove { index: usize },
    Event { index: usize, event: T::Event },
    Swap { index: usize, other: usize },
    Clear,
    Replace { values: Vec<T> },
}
