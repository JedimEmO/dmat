pub trait EventSourced {
    type Event;

    fn apply_event(&self, event: Self::Event);

    fn drain_events(&self, events: impl IntoIterator<Item = Self::Event>) {
        for event in events {
            self.apply_event(event);
        }
    }
}
