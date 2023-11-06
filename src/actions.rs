use crate::types::ActionType;

pub struct ActionQueue {
    pub queue: Vec<ActionType>,
}

impl ActionQueue {
    pub fn new() -> ActionQueue {
        ActionQueue { queue: Vec::new() }
    }
    pub fn enqueue(&mut self, action: ActionType) {
        self.queue.push(action);
    }

    pub fn drain(&mut self) -> std::vec::Drain<ActionType> {
        self.queue.drain(..)
    }
}
