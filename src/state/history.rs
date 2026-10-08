use crate::messages::Message;
use crate::state::memento::Memento;

const MAXIMUM_HISTORY_LENGTH: usize = 10000;

#[derive(Debug)]
pub struct History {
    undo_stack: Vec<Memento>,
    redo_stack: Vec<Memento>,
}

impl History {
    #[must_use]
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, item: Memento) {
        self.undo_stack.push(item);
        self.redo_stack.clear();
        if self.undo_stack.len() > MAXIMUM_HISTORY_LENGTH {
            // If the undo stack grows to large, delete the oldest entry.
            self.undo_stack.remove(0);
        }
    }

    #[must_use]
    pub fn recent<'a>(&'a mut self) -> Option<&'a mut Memento> {
        self.undo_stack.last_mut()
    }

    pub fn clear_redo(&mut self) {
        self.redo_stack.clear();
    }

    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    #[must_use]
    pub fn undo_message(&self) -> Option<Message> {
        if self.can_undo() {
            Some(Message::Undo)
        } else {
            None
        }
    }

    #[must_use]
    pub fn undo(&mut self) -> Option<Memento> {
        let last = self.undo_stack.pop();
        if last.is_some() {
            self.redo_stack.push(last.clone().unwrap());
        }
        last
    }

    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    #[must_use]
    pub fn redo_message(&self) -> Option<Message> {
        if self.can_redo() {
            Some(Message::Redo)
        } else {
            None
        }
    }

    #[must_use]
    pub fn redo(&mut self) -> Option<Memento> {
        let last = self.redo_stack.pop();
        if last.is_some() {
            self.undo_stack.push(last.clone().unwrap());
        }
        last
    }
}

impl Default for History {
    fn default() -> Self {
        History::new()
    }
}
