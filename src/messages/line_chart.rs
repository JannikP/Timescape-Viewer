#[derive(Debug, Clone)]
pub enum LineChartMessage {

    /// Sets the visibility of a signal.
    /// The signal is identified index of the signal (`signal`). The variable `visible` signifies
    /// the new, updated state. For example if the signal is currently visible and the user clicks
    /// "hide" the value of `visible` would be `false`.
    ToggleVisibility { signal: usize, visible: bool }
}
