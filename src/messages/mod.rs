#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_size_small() {
        assert!(std::mem::size_of::<Message>() <= 32);
    }
}
