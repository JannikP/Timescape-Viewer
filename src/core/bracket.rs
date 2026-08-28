use std::f64;

use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Bracket {
    pub span: Span,
    pub samples: u64,
    pub maximum: f64,
    pub minimum: f64,
    pub average: f64,
}

impl Bracket {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            samples: 0,
            maximum: f64::NEG_INFINITY,
            minimum: f64::INFINITY,
            average: 0.0,
        }
    }

    pub fn add(&mut self, sample: f64) {
        self.maximum = sample.max(self.maximum);
        self.minimum = sample.min(self.minimum);
        let n = self.samples as f64;
        self.average = (n * self.average + sample) / (n + 1.0);
        self.samples += 1;
    }
}
