/// A span of time with a beginning and an end in nanoseconds since a (undefined) reference point in
/// time. To have unambiguous assignment of time stamps to spans the beginning is inclusive and the
/// end is exclusive. A timestamp is considered inside a span if beginning <= sample < end.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
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
            Self { begin: self.begin, end: middle },
            Self { begin: middle, end: self.end },
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
