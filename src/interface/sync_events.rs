use std::{sync::{Arc, Mutex}, any::Any};

use sdl2::EventSubsystem;

#[derive(Clone)]
pub struct SyncEvents(Arc<Mutex<EventSubsystem>>);

impl SyncEvents {
    pub fn register_custom_event<T: Any>(&self) -> Result<(), String> {
        self.0
            .lock().unwrap()
            .register_custom_event::<T>()
    }

    pub fn push_custom_event(&self, event: impl Any) -> Result<(), String> {
        self.0
            .lock().unwrap()
            .push_custom_event(event)
    }
}

impl From<EventSubsystem> for SyncEvents {
    fn from(subsystem: EventSubsystem) -> Self {
        Self(Arc::new(Mutex::new(subsystem)))
    }
}
