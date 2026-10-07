#[derive(Debug, Clone)]
pub enum LineChartMessage {
    /// Sets the visibility of a signal.
    /// The signal is identified index of the signal (`signal`). The variable `visible` signifies
    /// the new, updated state. For example if the signal is currently visible and the user clicks
    /// "hide" the value of `visible` would be `false`.
    ToggleVisibility { signal: usize, visible: bool },

    /// Deletes a signal from the line chart. The signal to be deleted is identified by its index.
    RemoveSignal(usize),

    /// Some text is typed into the new signal input field.
    SignalInputChanged(String),

    /// Signal input field is focused and enter key is pressed.
    SignalInputSubmit,

    /// Zoom. The focal point (time/value) should remain at the same place on the screen.
    /// The time and value domains can be scrolled independently or together. The widget
    /// handles the decision what to keep and what to zoom based on the event and based on
    /// wether CTRL and/or SHIFT keys are held down. This event applies to all three
    /// cases.
    Zoom {
        /// Which physical value (y-axis) should remain stationary.
        focal_point: f32,

        /// How much to zoom the physical domain. A value of `1.0` indicates no change, a value
        /// 0..1.0 zooms in and a value > 1.0 zooms out (shows more).
        zoom: f32,
    },
}
