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
    pub fn new(begin: i64, end: i64) -> Self {
        assert!(end > begin, "A span's end must be after its begin.");
        Self { begin, end }
    }

    pub fn is_zero(&self) -> bool {
        self.begin + 1 < self.end
    }

    pub fn contains(&self, timestamp: i64) -> bool {
        self.begin <= timestamp && timestamp < self.end
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
