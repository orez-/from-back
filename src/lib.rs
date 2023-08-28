use std::ops::*;

// ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeqIndex<Idx> {
    FromFront(Idx),
    FromBack(Idx),
}

impl SeqIndex<usize> {
    pub fn for_seq_len(&self, len: usize) -> usize {
        match self {
            &SeqIndex::FromFront(idx) => idx,
            &SeqIndex::FromBack(idx) => len.checked_sub(idx).unwrap(),
        }
    }
}

impl<Idx> Default for SeqIndex<Idx>
where Idx: Default {
    fn default() -> Self {
        SeqIndex::FromFront(Default::default())
    }
}

// ===

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRange<Idx> {
    pub start: SeqIndex<Idx>,
    pub end: SeqIndex<Idx>,
}

impl SeqRange<usize> {
    pub fn for_seq_len(&self, len: usize) -> Range<usize> {
        self.start.for_seq_len(len)..self.end.for_seq_len(len)
    }
}

// ===

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRangeFrom<Idx> {
    pub start: SeqIndex<Idx>,
}

impl SeqRangeFrom<usize> {
    pub fn for_seq_len(&self, len: usize) -> RangeFrom<usize> {
        self.start.for_seq_len(len)..
    }
}

// ===

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRangeInclusive<Idx> {
    pub start: SeqIndex<Idx>,
    pub end: SeqIndex<Idx>,
}

impl SeqRangeInclusive<usize> {
    pub fn for_seq_len(&self, len: usize) -> RangeInclusive<usize> {
        self.start.for_seq_len(len)..=self.end.for_seq_len(len)
    }
}

// ===
// Vec impls

impl<T> Index<SeqIndex<usize>> for Vec<T> {
    type Output = T;

    fn index(&self, rng: SeqIndex<usize>) -> &T {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRange<usize>> for Vec<T> {
    type Output = [T];

    fn index(&self, rng: SeqRange<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRangeFrom<usize>> for Vec<T> {
    type Output = [T];

    fn index(&self, rng: SeqRangeFrom<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRangeInclusive<usize>> for Vec<T> {
    type Output = [T];

    fn index(&self, rng: SeqRangeInclusive<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

// ===
// Slice impls

impl<T> Index<SeqIndex<usize>> for [T] {
    type Output = T;

    fn index(&self, rng: SeqIndex<usize>) -> &T {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRange<usize>> for [T] {
    type Output = [T];

    fn index(&self, rng: SeqRange<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRangeFrom<usize>> for [T] {
    type Output = [T];

    fn index(&self, rng: SeqRangeFrom<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl<T> Index<SeqRangeInclusive<usize>> for [T] {
    type Output = [T];

    fn index(&self, rng: SeqRangeInclusive<usize>) -> &[T] {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

// ===
// str impls

impl Index<SeqRange<usize>> for str {
    type Output = str;

    fn index(&self, rng: SeqRange<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl Index<SeqRangeFrom<usize>> for str {
    type Output = str;

    fn index(&self, rng: SeqRangeFrom<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl Index<SeqRangeInclusive<usize>> for str {
    type Output = str;

    fn index(&self, rng: SeqRangeInclusive<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

// ===
// String impls

impl Index<SeqRange<usize>> for String {
    type Output = str;

    fn index(&self, rng: SeqRange<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl Index<SeqRangeFrom<usize>> for String {
    type Output = str;

    fn index(&self, rng: SeqRangeFrom<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

impl Index<SeqRangeInclusive<usize>> for String {
    type Output = str;

    fn index(&self, rng: SeqRangeInclusive<usize>) -> &str {
        let range = rng.for_seq_len(self.len());
        &self[range]
    }
}

// ===

#[macro_export]
macro_rules! idx {
    // we allow specifying a `idx!(..)` for completeness, but..
    // ..there's no need to create a custom type for it.
    ( .. ) => { .. };
    ( $left:tt..$right:tt ) => { SeqRange {
        start: SeqIndex::FromFront($left),
        end: SeqIndex::FromFront($right),
    } };
    ( ^$left:tt..$right:tt ) => { SeqRange {
        start: SeqIndex::FromBack($left),
        end: SeqIndex::FromFront($right),
    } };
    ( $left:tt..^$right:tt ) => { SeqRange {
        start: SeqIndex::FromFront($left),
        end: SeqIndex::FromBack($right),
    } };
    ( ^$left:tt..^$right:tt ) => { SeqRange {
        start: SeqIndex::FromBack($left),
        end: SeqIndex::FromBack($right),
    } };
    ( $left:tt.. ) => { SeqRangeFrom {
        start: SeqIndex::FromFront($left),
    } };
    ( ^$left:tt.. ) => { SeqRangeFrom {
        start: SeqIndex::FromBack($left),
    } };
    ( $left:tt..=$right:tt ) => { SeqRangeInclusive {
        start: SeqIndex::FromFront($left),
        end: SeqIndex::FromFront($right),
    } };
    ( ^$left:tt..=$right:tt ) => { SeqRangeInclusive {
        start: SeqIndex::FromBack($left),
        end: SeqIndex::FromFront($right),
    } };
    ( $left:tt..=^$right:tt ) => { SeqRangeInclusive {
        start: SeqIndex::FromFront($left),
        end: SeqIndex::FromBack($right),
    } };
    ( ^$left:tt..=^$right:tt ) => { SeqRangeInclusive {
        start: SeqIndex::FromBack($left),
        end: SeqIndex::FromBack($right),
    } };
    ( ..$right:tt ) => { SeqRange {
        start: Default::default(),
        end: SeqIndex::FromFront($right),
    } };
    ( ..^$right:tt ) => { SeqRange {
        start: Default::default(),
        end: SeqIndex::FromBack($right),
    } };
    ( ..=$right:tt ) => { SeqRangeInclusive {
        start: Default::default(),
        end: SeqIndex::FromFront($right),
    } };
    ( ..=^$right:tt ) => { SeqRangeInclusive {
        start: Default::default(),
        end: SeqIndex::FromBack($right),
    } };
    ( ^$x:expr ) => { SeqIndex::FromBack($x) };
    ( $x:expr ) => { SeqIndex::FromFront($x) };
}

// ===

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_slice() {
        let range = SeqRange {
            start: SeqIndex::FromFront(2),
            end: SeqIndex::FromBack(3),
        };
        let vec: Vec<_> = (0..10).collect();
        let slice: &[_] = &vec;
        assert_eq!(&slice[range], &[2, 3, 4, 5, 6]);
    }

    #[test]
    fn index_vec() {
        let range = SeqRange {
            start: SeqIndex::FromFront(2),
            end: SeqIndex::FromBack(3),
        };
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[range], [2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_from_right_macro() {
        let idx = idx!(^5);
        assert!(matches!(idx, SeqIndex::FromBack(5)));
    }

    #[test]
    fn test_from_left_macro() {
        let idx = idx!(5);
        assert!(matches!(idx, SeqIndex::FromFront(5)));
    }

    #[test]
    fn test_range_macro() {
        let idx = idx!(2..^3);
        assert!(matches!(idx, SeqRange {
            start: SeqIndex::FromFront(2),
            end: SeqIndex::FromBack(3),
        }), "{idx:?}");
    }

    #[test]
    fn test_macro() {
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[idx!(2..^3)], [2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_homogeneous_macro_ranges() {
        // exercises that all these ranges can go in the `vec` together,
        // that we didn't cut corners and return a `Range` for `idx!(2..7)`
        let ranges = vec![
            idx!(2..7),
            idx!(2..^3),
            idx!(^8..7),
            idx!(^8..^3),
        ];
        let vec: Vec<_> = (0..10).collect();
        for range in ranges {
            assert_eq!(vec[range], [2, 3, 4, 5, 6]);
        }
    }

    #[test]
    fn test_full_range() {
        assert_eq!(idx!(..), ..);
    }

    #[test]
    fn test_str() {
        assert_eq!(&"ranges"[idx!(1..^2)], "ang");
    }

    #[test]
    fn test_string() {
        let s = "ranges".to_string();
        assert_eq!(&s[idx!(1..^2)], "ang");
    }

    #[test]
    fn test_range_to() {
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[idx!(..^3)], [0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_range_to_incl() {
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[idx!(..=^3)], [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
