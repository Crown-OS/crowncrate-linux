use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{actions::action::Action, communication::{Actions, Message}};

pub struct ActionManager {
    actions: Arc<Mutex<HashMap<Actions, Box<dyn Action>>>>
}

impl ActionManager {
    pub fn new() -> Self {
        ActionManager {
            actions: Arc::new(Mutex::new(HashMap::default()))
        }
    }

    pub fn subscribe(&mut self, action_type: Actions, action: Box<dyn Action>) {
        let binding = self.actions.clone();
        if let Ok(mut actions) = binding.lock() {
            actions.insert(action_type, action);
        }
    }

    pub fn unsubscribe(&mut self, action: Actions) {
        let binding = self.actions.clone();
        if let Ok(mut actions) = binding.lock() {
            actions.retain(|&i, _| {
                 i == action
            });
        }
    }
    
    pub fn notify(&self, message: Message) {
        for action in self.actions {
            
        }
    }
}
