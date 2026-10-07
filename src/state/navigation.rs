pub struct Navigator {}

impl Navigator {
    pub fn can_undo(&self) -> bool {
        false
    }

    pub fn undo(&mut self) {}

    pub fn can_redo(&self) -> bool {
        false
    }

    pub fn redo(&mut self) {}
}
