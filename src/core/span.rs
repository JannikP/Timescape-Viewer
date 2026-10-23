use glam::{Vec4, vec4};

/// A span of time with a beginning and an end in nanoseconds since a (undefined) reference point in
/// time. To have unambiguous assignment of time stamps to spans the beginning is inclusive and the
/// end is exclusive. A timestamp is considered inside a span if beginning <= sample < end.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    pub begin: i64,
    pub end: i64,
}

impl Span {
    #[must_use]
    pub fn new(begin: i64, end: i64) -> Self {
        assert!(end > begin, "A span's end must be after its begin.");
        Self { begin, end }
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.begin + 1 < self.end
    }

    #[must_use]
    pub fn contains(&self, timestamp: i64) -> bool {
        self.begin <= timestamp && timestamp < self.end
    }

    /// Computes the midpoint of this [Span].
    #[must_use]
    pub fn middle(&self) -> i64 {
        (self.begin + self.end) / 2
    }

    /// Performs a zoom action on this [Span]. The `pivot` point (timestamp) will remain stationary
    /// while the range zooms in or out around it. A scaling `factor` of 0.0..1.0 zooms in, so a
    /// smaller [Span] will be visible and a factor larger than 1.0 zooms out.
    /// If the function is called with a `factor` of 1.0, nothing changes.
    pub fn zoom(&self, pivot: i64, factor: f32) -> Span {
        let before = (self.begin - pivot) as f64 * factor as f64;
        let after = (self.end - pivot) as f64 * factor as f64;
        let begin = pivot + before.round() as i64;
        let end = pivot + after.round() as i64;
        Span::new(begin, end)
    }

    /// How much to shift the time [Span] (time-axis). A positive value increases the begin and end
    /// values, causing the trace lines to move left on screen. The translation between mouse motion
    /// and time values is handled by the widget.
    pub fn pan(&self, delta: f32) -> Span {
        let delta = delta.round() as i64;
        Span::new(self.begin + delta, self.end + delta)
    }

    pub fn half(&self) -> (Self, Self) {
        let middle = (self.begin + self.end) / 2;
        (
            Self {
                begin: self.begin,
                end: middle,
            },
            Self {
                begin: middle,
                end: self.end,
            },
        )
    }

    pub fn split(&self, count: usize) -> Vec<Span> {
        let mut splits = Vec::with_capacity(count);
        let mut begin = self.begin;
        let duration = self.end - self.begin;
        let count = count as i64;
        for i in 1..=count {
            let end = duration * i / count;
            splits.push(Span::new(begin, end));
            begin = end;
        }
        splits
    }
}

impl From<Vec4> for Span {
    fn from(value: Vec4) -> Self {
        let begin = value.x.round() as i64;
        let end = (value.y.round() as i64).max(begin + 1);
        Span::new(begin, end)
    }
}

impl Into<Vec4> for &Span {
    fn into(self) -> Vec4 {
        // FIXME: There is a significant loss of precision as f32 has only 24 bits of mantissa compared to i64's 63 bits.
        vec4(self.begin as f32, self.end as f32, 0.0, 0.0)
    }
}
