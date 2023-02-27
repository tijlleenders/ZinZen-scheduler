use std::collections::HashMap;

use crate::task::Task;
/// An event type.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Schedule,
}

/// Budget sends events to Tasks (listeners).
#[derive(Default)]
pub struct Budget {
    events: HashMap<Event, Vec<Task>>,
}

impl Budget {
    pub fn subscribe(&mut self, event_type: Event, listener: Task) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Task) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|x| x.id != listener.id);
    }

    pub fn notify(&self, event_type: Event) {
        let Tasks = self.events.get(&event_type).unwrap();
        for Task in Tasks {}
    }
}
