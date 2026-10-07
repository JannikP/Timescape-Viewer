use crate::state::Timestamp;

#[derive(Debug, Clone)]
pub enum Message {
    /// Zoom. The focal point (time/value) should remain at the same place on the screen.
    /// The time and value domains can be scrolled independently or together. The widget
    /// handles the decision what to keep and what to zoom based on the event and based on
    /// wether CTRL and/or SHIFT keys are held down. This event applies to all three
    /// cases.
    Zoom {
        /// Which timestamp should remain stationary.
        focal_point: Timestamp,

        /// How much to zoom the time domain. A value of `1.0` indicates no change, a value
        /// 0..1.0 zooms in and a value > 1.0 zooms out (shows more).
        zoom: f32,
    },

    /// Pan
    Pan {
        /// How much to shift the time range. A positive value increases the starting time,
        /// causing the visible features to move left on screen. The translation between
        /// mouse motion and time values is handled by the widget.
        delta: i64,

        /// How much to shift the physical value range (y-axis). A positive value increases
        /// the minimum and maximum values, causing the trace lines to move down on screen.
        /// The translation between mouse motion and physical values is handled by the widget.
        delta_value: f32,
    },

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
