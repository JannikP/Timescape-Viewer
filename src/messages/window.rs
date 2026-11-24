#[derive(Debug, Clone)]
pub enum Message {
    /// Show and place cursor.
    Place { cursor: Cursor, position: i64 },

    /// Hide cursor.
    Dismiss(Cursor),

    /// Minimize,
    Minimize,

    /// Restore,
    Restore,

    /// Maximize,
    Maximize,

    /// Close the window entirely.
    Close,
}

#[derive(Debug, Clone)]
pub enum Cursor {
    Primary,
    Secondary,
}
