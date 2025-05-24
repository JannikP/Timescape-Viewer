use crate::origins::Origin;
use crate::state::Stage;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    GoTo(Stage),
    ChooseFile,
    Open(Origin),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_size_small() {
        assert!(std::mem::size_of::<Message>() <= 32);
    }
}
