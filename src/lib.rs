use std::ops::*;

// ===

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyRange<Idx> {
    pub start: RangeIndex<Idx>,
    pub end: RangeIndex<Idx>,
}

impl MyRange<usize> {
    pub fn for_slice_len(&self, len: usize) -> Range<usize> {
        self.start.for_slice_len(len)..self.end.for_slice_len(len)
    }
}

// ===

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyRangeFrom<Idx> {
    pub start: RangeIndex<Idx>,
}

impl MyRangeFrom<usize> {
    pub fn for_slice_len(&self, len: usize) -> RangeFrom<usize> {
        self.start.for_slice_len(len)..
    }
}

// ===

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MyRangeInclusive<Idx> {
    pub start: RangeIndex<Idx>,
    pub end: RangeIndex<Idx>,
}

impl MyRangeInclusive<usize> {
    pub fn for_slice_len(&self, len: usize) -> RangeInclusive<usize> {
        self.start.for_slice_len(len)..=self.end.for_slice_len(len)
    }
}

// ===

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RangeIndex<Idx> {
    FromFront(Idx),
    FromBack(Idx),
}

impl RangeIndex<usize> {
    fn for_slice_len(&self, len: usize) -> usize {
        match self {
            &RangeIndex::FromFront(idx) => idx,
            &RangeIndex::FromBack(idx) => len.checked_sub(idx).unwrap(),
        }
    }
}

impl<Idx> Default for RangeIndex<Idx>
where Idx: Default {
    fn default() -> Self {
        RangeIndex::FromFront(Default::default())
    }
}

// ===

impl<T> Index<MyRange<usize>> for Vec<T> {
    type Output = [T];

    fn index(&self, rng: MyRange<usize>) -> &[T] {
        let range = rng.for_slice_len(self.len());
        &self[range]
    }
}

impl<T> Index<MyRange<usize>> for [T] {
    type Output = [T];

    fn index(&self, rng: MyRange<usize>) -> &[T] {
        let range = rng.for_slice_len(self.len());
        &self[range]
    }
}

#[macro_export]
macro_rules! idx {
    // we allow specifying a `idx!(..)` for completeness, but..
    // ..there's no need to create a custom type for it.
    ( .. ) => { .. };
    ( $left:tt..$right:tt ) => { MyRange {
        start: RangeIndex::FromFront($left),
        end: RangeIndex::FromFront($right),
    } };
    ( ^$left:tt..$right:tt ) => { MyRange {
        start: RangeIndex::FromBack($left),
        end: RangeIndex::FromFront($right),
    } };
    ( $left:tt..^$right:tt ) => { MyRange {
        start: RangeIndex::FromFront($left),
        end: RangeIndex::FromBack($right),
    } };
    ( ^$left:tt..^$right:tt ) => { MyRange {
        start: RangeIndex::FromBack($left),
        end: RangeIndex::FromBack($right),
    } };
    ( $left:tt.. ) => { MyRangeFrom {
        start: RangeIndex::FromFront($left),
    } };
    ( ^$left:tt.. ) => { MyRangeFrom {
        start: RangeIndex::FromBack($left),
    } };
    ( $left:tt..=$right:tt ) => { MyRangeInclusive {
        start: RangeIndex::FromFront($left),
        end: RangeIndex::FromFront($right),
    } };
    ( ^$left:tt..=$right:tt ) => { MyRangeInclusive {
        start: RangeIndex::FromBack($left),
        end: RangeIndex::FromFront($right),
    } };
    ( $left:tt..=^$right:tt ) => { MyRangeInclusive {
        start: RangeIndex::FromFront($left),
        end: RangeIndex::FromBack($right),
    } };
    ( ^$left:tt..=^$right:tt ) => { MyRangeInclusive {
        start: RangeIndex::FromBack($left),
        end: RangeIndex::FromBack($right),
    } };
    ( ^$x:expr ) => { RangeIndex::FromBack($x) };
    ( $x:expr ) => { RangeIndex::FromFront($x) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_slice() {
        let range = MyRange {
            start: RangeIndex::FromFront(2),
            end: RangeIndex::FromBack(3),
        };
        let vec: Vec<_> = (0..10).collect();
        let slice: &[_] = &vec;
        assert_eq!(&slice[range], &[2, 3, 4, 5, 6]);
    }

    #[test]
    fn index_vec() {
        let range = MyRange {
            start: RangeIndex::FromFront(2),
            end: RangeIndex::FromBack(3),
        };
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[range], [2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_from_right_macro() {
        let idx = idx!(^5);
        assert!(matches!(idx, RangeIndex::FromBack(5)));
    }

    #[test]
    fn test_from_left_macro() {
        let idx = idx!(5);
        assert!(matches!(idx, RangeIndex::FromFront(5)));
    }

    #[test]
    fn test_range_macro() {
        let idx = idx!(2..^3);
        assert!(matches!(idx, MyRange {
            start: RangeIndex::FromFront(2),
            end: RangeIndex::FromBack(3),
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
}
