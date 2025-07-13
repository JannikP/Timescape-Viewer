use crate::origins::Origin;
use crate::state::Stage;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    AbortModal,
    ChooseFile,
    GoTo(Stage),
    Hint(String),
    Open(Origin),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_size_small() {
        assert!(
            std::mem::size_of::<Message>() <= 64,
            "The message should be less than or equal to 64 bytes in size. Box larger content!\nThe actual size is {} bytes.",
            std::mem::size_of::<Message>(),
        );
    }
}
