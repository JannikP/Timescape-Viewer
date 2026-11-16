/// Identifies one of the two cursors of a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorId {
    Primary,
    Secondary,
}
