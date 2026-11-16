use crate::core::CursorId;

#[derive(Debug, Clone)]
pub enum Message {
    /// Show and place cursor.
    Place {
        cursor: CursorId,
        position: i64,
    },

    /// Hide cursor.
    Dismiss(CursorId),

    /// Minimize,
    Minimize,

    /// Restore,
    Restore,

    /// Maximize,
    Maximize,

    /// Close the window entirely.
    Close,
}
