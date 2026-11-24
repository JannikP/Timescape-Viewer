use crate::state::line_chart::LineColor;

#[derive(Debug, Clone)]
pub enum LineChartMessage {
    /// Sets the visibility of a signal.
    /// The signal is identified index of the signal (`signal`). The variable `visible` signifies
    /// the new, updated state. For example if the signal is currently visible and the user clicks
    /// "hide" the value of `visible` would be `false`.
    ToggleVisibility { signal: usize, visible: bool },

    /// Show the color chooser for a signal.
    InvokeColorChooser { signal: usize },

    /// Sets the color of a signal.
    /// The color can be [LineColor::Automatic] assigned based on the current theme and signal index
    /// or [LineColor::UserDefined] and totally arbitrary.
    Color { signal: usize, color: LineColor },

    /// Deletes a signal from the line chart. The signal to be deleted is identified by its index.
    RemoveSignal(usize),

    /// Some text is typed into the new signal input field.
    SignalInputChanged(String),

    /// Signal input field is focused and enter key is pressed.
    SignalInputSubmit,
}
