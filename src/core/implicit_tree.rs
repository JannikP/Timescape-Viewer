//! Implicit In-order Forests implementation inspired by
//! [Tristan Hume's post](https://thume.ca/2021/03/14/iforests/).

use std::ops::Range;

pub trait Aggregate: Clone {
    fn empty() -> Self;

    fn combine(&self, other: &Self) -> Self;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Minium(f32);

impl Aggregate for Minium {
    fn empty() -> Self {
        Self(f32::INFINITY)
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
}

impl From<f32> for Minium {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Maximum(f32);

impl Aggregate for Maximum {
    fn empty() -> Self {
        Self(f32::NEG_INFINITY)
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl From<f32> for Maximum {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Sum(f32);

impl Aggregate for Sum {
    fn empty() -> Self {
        Self(0.0)
    }

    fn combine(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl From<f32> for Sum {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct IForestIndex<A> {
    values: Vec<A>
}

impl<A> IForestIndex<A> where A: Aggregate {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { values: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, value: impl Into<A>) {
        self.values.push(value.into());

        let len = self.values.len();
        // We want to index the first level every 2 nodes, 2nd level every 4 nodes...
        // This happens to correspond to the number of trailing ones in the index
        let levels_to_index = len.trailing_ones()-1;

        // Complete unfinished aggregation nodes which are now ready
        let mut cur = len-1; // The leaf we just pushed
        for level in 0..levels_to_index {
            let prev_higher_level = cur-(1 << level); // nodes at a level reach 2^level
            let combined = A::combine(&self.values[prev_higher_level], &self.values[cur]);
            self.values[prev_higher_level] = combined;
            cur = prev_higher_level;
        }

        // Push new aggregation node going back one level further than we aggregated
        self.values.push(self.values[len-(1 << levels_to_index)].clone());
    }

    // TODO: pub fn aggregate(&mut self, index: usize, value: impl Into<A>) { }

    // TODO: pub fn replace(&mut self, index: usize, value: impl Into<A>) { }

    /// Shrinks the capacity of the vector as much as possible.
    ///
    /// The behavior of this method depends on the allocator, which may
    /// either shrink the vector in-place or reallocate. The resulting
    /// vector might still have some excess capacity, just as is the case for
    /// [IForestIndex::with_capacity].
    pub fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    // TODO: pub fn len(&self) -> usize

    // TODO: pub fn is_empty(&self) -> bool

    /// See [havelessbemore's explanation] for more on why these bit tricks
    /// work. Thanks to him for the enhanced bit tricks for fewer branches.
    ///
    /// [havelessbemore's explanation]: https://github.com/havelessbemore/dastal/blob/cd6a1d03872aa437f9272ce3fd42e2e2c006b2cc/src/segmentTree/inOrderSegmentTree.ts
    pub fn query(&self, r: Range<usize>) -> A {
        /// offset past largest tree with left index x
        fn lsp(x: usize) -> usize {
            x & x.wrapping_neg() // leave the least significant bit
        }
        /// offset past largest tree up to x long
        fn msp(x: usize) -> usize {
            1usize.reverse_bits() >> x.leading_zeros() // leave the most significant bit
        }
        fn largest_prefix_inside_skip(min: usize, max: usize) -> usize {
            lsp(min|msp(max-min)) // = usize::min(lsp(min),msp(max-min))
        }
        fn agg_node(i: usize, offset: usize) -> usize {
            i + (offset >> 1) - 1 //
        }

        let mut ri = (r.start*2)..(r.end*2); // translate underlying to interior indices
        let len = self.values.len();
        assert!(ri.start <= len && ri.end <= len, "range {:?} not inside 0..{}", r, len/2);

        let mut combined = A::empty();
        while ri.start < ri.end {
            let skip = largest_prefix_inside_skip(ri.start, ri.end);
            combined = A::combine(&combined, &self.values[agg_node(ri.start, skip)]);
            ri.start += skip
        }
        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_min() {
        // Arrange
        let mut forest: IForestIndex<Minium> = IForestIndex::with_capacity(2);

        // Act
        forest.push(1.0);
        forest.push(2.0);

        // Assert
        assert_eq!(forest.values, vec![Minium(1.0), Minium(1.0), Minium(2.0), Minium(1.0)]);
    }

    #[test]
    fn test_push_max() {
        // Arrange
        let mut forest: IForestIndex<Maximum> = IForestIndex::with_capacity(2);

        // Act
        forest.push(1.0);
        forest.push(2.0);

        // Assert
        assert_eq!(forest.values, vec![Maximum(1.0), Maximum(2.0), Maximum(2.0), Maximum(2.0)]);
    }

    #[test]
    fn test_push_sum() {
        // Arrange
        let mut forest: IForestIndex<Sum> = IForestIndex::with_capacity(2);

        // Act
        forest.push(1.0);
        forest.push(2.0);

        // Assert
        assert_eq!(forest.values, vec![Sum(1.0), Sum(3.0), Sum(2.0), Sum(3.0)]);
    }

    #[test]
    fn test_query_full_range_sum() {
        // Arrange
        let mut forest: IForestIndex<Sum> = IForestIndex::with_capacity(4);
        for i in 1..=4 {
            forest.push(i as f32);
        }

        // Act
        let result = forest.query(0..4);

        // Assert
        assert_eq!(result, Sum(10.0));
    }

    #[test]
    fn test_query_sub_range_sum() {
        // Arrange
        let mut forest: IForestIndex<Sum> = IForestIndex::with_capacity(4);
        for i in 1..=4 {
            forest.push(i as f32);
        }

        // Act
        let result = forest.query(0..3);

        // Assert
        assert_eq!(result, Sum(6.0));
    }
}
