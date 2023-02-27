use std::collections::HashMap;

use crate::{Goal, goal};

/// An event type.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Schedule,
}


/// Budget sends events to Goals (listeners).
#[derive(Default)]
pub struct Budget {
    events: HashMap<Event, Vec<Goal>>,
}

impl Budget {
    pub fn subscribe(&mut self, event_type: Event, listener: Goal) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Goal) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|x| x.id != listener.id);
    }

    pub fn notify(&self, event_type: Event) {
        let goals = self.events.get(&event_type).unwrap();
        for goal in goals {
            
        }
    }
}