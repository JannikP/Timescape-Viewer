use crate::state::Stage;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GoTo(Stage),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_size_small() {
        assert!(std::mem::size_of::<Message>() <= 32);
    }
}
