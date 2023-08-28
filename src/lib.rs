use std::ops::{Index, Range};

#[derive(Debug)]
pub struct MyRange<T> {
    pub start: T,
    pub end: T,
}

impl MyRange<RangeIndex<usize>> {
    pub fn for_slice_len(&self, len: usize) -> Range<usize> {
        self.start.for_slice_len(len)..self.end.for_slice_len(len)
    }
}

#[derive(Debug)]
pub enum RangeIndex<T> {
    FromFront(T),
    FromBack(T),
}

impl RangeIndex<usize> {
    fn for_slice_len(&self, len: usize) -> usize {
        match self {
            &RangeIndex::FromFront(idx) => idx,
            &RangeIndex::FromBack(idx) => len.checked_sub(idx).unwrap(),
        }
    }
}

impl<T> Index<MyRange<RangeIndex<usize>>> for Vec<T> {
    type Output = [T];

    fn index(&self, rng: MyRange<RangeIndex<usize>>) -> &[T] {
        let range = rng.for_slice_len(self.len());
        &self[range]
    }
}

impl<T> Index<MyRange<RangeIndex<usize>>> for [T] {
    type Output = [T];

    fn index(&self, rng: MyRange<RangeIndex<usize>>) -> &[T] {
        let range = rng.for_slice_len(self.len());
        &self[range]
    }
}

#[macro_export]
macro_rules! range {
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
        let idx = range!(^5);
        assert!(matches!(idx, RangeIndex::FromBack(5)));
    }

    #[test]
    fn test_from_left_macro() {
        let idx = range!(5);
        assert!(matches!(idx, RangeIndex::FromFront(5)));
    }

    #[test]
    fn test_range_macro() {
        let idx = range!(2..^3);
        assert!(matches!(idx, MyRange {
            start: RangeIndex::FromFront(2),
            end: RangeIndex::FromBack(3),
        }), "{idx:?}");
    }

    #[test]
    fn test_macro() {
        let vec: Vec<_> = (0..10).collect();
        assert_eq!(vec[range!(2..^3)], [2, 3, 4, 5, 6]);
    }
}
