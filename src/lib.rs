//! Library for indexing and slicing from the back of a sequence.
//!
//! The main entrypoint for this library is the [`idx!`] macro, which
//! generates the library's structs. These structs can be used to index
//! into common sequence types, notably [`Vec`] and [`slice`].
//! The [`idx!`] macro exposes a virtual unary operator `^`, which
//! indicates the index should count from the back of the sequence
//! instead of the front.
//!
//! # Examples
//!
//! ```rust
//! # use from_back::idx;
//! let vec = vec![8, 6, 7, 5, 3, 0, 9];
//! // the element second from the back is 0
//! assert_eq!(vec[idx!(^2)], 0);
//! // slice the elements two from the front to three from the back (exclusive)
//! assert_eq!(&vec[idx!(2..^3)], &[7, 5]);
//! // slice the elements two from the front to three from the back (inclusive)
//! assert_eq!(&vec[idx!(2..=^3)], &[7, 5, 3]);
//! ```

use std::ops::*;

// ===

/// Container type for indexing from the front or back of a sequence (`idx!(index)`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeqIndex<Idx> {
    /// The index counts from the front of the sequence.
    FromFront(Idx),
    /// The index counts from the back of the sequence.
    FromBack(Idx),
}

impl SeqIndex<usize> {
    /// Convert this container to a native from-front [`usize`] for a sequence of the given `len`.
    ///
    /// If the value is `FromFront(index)`, returns `index`.
    /// If the value is `FromBack(index)`, returns `len - index`.
    ///
    /// # Panics
    ///
    /// Panics if the from-back value exceeds the given length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use from_back::idx;
    /// let vec = vec![8, 6, 7, 5, 3, 0, 9];
    /// let index = idx!(^2).for_seq_len(vec.len());
    /// assert_eq!(index, 5);
    /// assert_eq!(vec.get(index), Some(&0));
    /// ```
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

/// A parallel to [`std::ops::Range`] (`idx!(start..end)`)
///
/// # Examples
/// ```rust
/// # use from_back::idx;
/// let vec = vec![8, 6, 7, 5, 3, 0, 9];
///
/// // slice the elements two from the front to three from the back
/// assert_eq!(&vec[idx!(2..^3)], &[7, 5]);
///
/// // slice the elements five from the back to four from the front
/// assert_eq!(&vec[idx!(^5..4)], &[7, 5]);
///
/// // slice the elements two from the back to zero from the back (the back itself)
/// assert_eq!(&vec[idx!(^2..^0)], &[0, 9]);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRange<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: SeqIndex<Idx>,
    /// The upper bound of the range (exclusive).
    pub end: SeqIndex<Idx>,
}

impl SeqRange<usize> {
    /// Convert this container to a native [`Range`].
    ///
    /// Delegates to each index's [`SeqIndex::for_seq_len`].
    ///
    /// # Panics
    ///
    /// Panics if either index's conversion panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use from_back::idx;
    /// let vec = vec![8, 6, 7, 5, 3, 0, 9];
    /// let range = idx!(2..^2).for_seq_len(vec.len());
    /// let expected: &[_] = &[7, 5, 3];
    /// assert_eq!(range, 2..5);
    /// assert_eq!(vec.get(range), Some(expected));
    /// ```
    pub fn for_seq_len(&self, len: usize) -> Range<usize> {
        self.start.for_seq_len(len)..self.end.for_seq_len(len)
    }
}

// ===

/// A parallel to [`std::ops::RangeFrom`] (`idx!(start..)`)
///
/// # Examples
/// ```rust
/// # use from_back::idx;
/// let vec = vec![8, 6, 7, 5, 3, 0, 9];
///
/// // slice the elements starting two from the back
/// assert_eq!(&vec[idx!(^2..)], &[0, 9]);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRangeFrom<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: SeqIndex<Idx>,
}

impl SeqRangeFrom<usize> {
    /// Convert this container to a native [`RangeFrom`].
    ///
    /// Delegates to the `start` index's [`SeqIndex::for_seq_len`].
    ///
    /// # Panics
    ///
    /// Panics if the `start` index's conversion panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use from_back::idx;
    /// let vec = vec![8, 6, 7, 5, 3, 0, 9];
    /// let range = idx!(^2..).for_seq_len(vec.len());
    /// let expected: &[_] = &[0, 9];
    /// assert_eq!(range, 5..);
    /// assert_eq!(vec.get(range), Some(expected));
    /// ```
    pub fn for_seq_len(&self, len: usize) -> RangeFrom<usize> {
        self.start.for_seq_len(len)..
    }
}

// ===

/// A parallel to [`std::ops::RangeInclusive`] (`idx!(start..=end)`)
///
/// # Examples
/// ```rust
/// # use from_back::idx;
/// let vec = vec![8, 6, 7, 5, 3, 0, 9];
///
/// // slice the elements two from the front to three from the back, inclusive
/// assert_eq!(&vec[idx!(2..=^3)], &[7, 5, 3]);
///
/// // slice the elements five from the back to four from the front, inclusive
/// assert_eq!(&vec[idx!(^5..=4)], &[7, 5, 3]);
///
/// // slice the elements two from the back to one from the back, inclusive
/// assert_eq!(&vec[idx!(^2..=^1)], &[0, 9]);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SeqRangeInclusive<Idx> {
    /// The lower bound of the range (inclusive).
    pub start: SeqIndex<Idx>,
    /// The upper bound of the range (inclusive).
    pub end: SeqIndex<Idx>,
}

impl SeqRangeInclusive<usize> {
    /// Convert this container to a native [`RangeInclusive`].
    ///
    /// Delegates to each index's [`SeqIndex::for_seq_len`].
    ///
    /// # Panics
    ///
    /// Panics if either index's conversion panics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use from_back::idx;
    /// let vec = vec![8, 6, 7, 5, 3, 0, 9];
    /// let range = idx!(2..=^2).for_seq_len(vec.len());
    /// let expected: &[_] = &[7, 5, 3, 0];
    /// assert_eq!(range, 2..=5);
    /// assert_eq!(vec.get(range), Some(expected));
    /// ```
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

/// Create an index or range which may have "from back" components.
///
/// Generates structs which can be used to index
/// into common sequence types, notably [`Vec`] and [`slice`].
/// This macro exposes a virtual unary operator `^`, which
/// indicates the index should count from the back of the sequence
/// instead of the front.
///
/// # Examples
///
/// ```rust
/// # use from_back::idx;
/// let vec = vec![8, 6, 7, 5, 3, 0, 9];
/// // the element second from the back is 0
/// assert_eq!(vec[idx!(^2)], 0);
/// // slice the elements two from the front to three from the back (exclusive)
/// assert_eq!(&vec[idx!(2..^3)], &[7, 5]);
/// // slice the elements two from the front to three from the back (inclusive)
/// assert_eq!(&vec[idx!(2..=^3)], &[7, 5, 3]);
/// ```
#[macro_export]
macro_rules! idx {
    // we allow specifying a `idx!(..)` for completeness, but..
    // ..there's no need to create a custom type for it.
    ( .. ) => { .. };
    ( $left:tt..$right:tt ) => { $crate::SeqRange {
        start: $crate::SeqIndex::FromFront($left),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( ^$left:tt..$right:tt ) => { $crate::SeqRange {
        start: $crate::SeqIndex::FromBack($left),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( $left:tt..^$right:tt ) => { $crate::SeqRange {
        start: $crate::SeqIndex::FromFront($left),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( ^$left:tt..^$right:tt ) => { $crate::SeqRange {
        start: $crate::SeqIndex::FromBack($left),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( $left:tt.. ) => { $crate::SeqRangeFrom {
        start: $crate::SeqIndex::FromFront($left),
    } };
    ( ^$left:tt.. ) => { $crate::SeqRangeFrom {
        start: $crate::SeqIndex::FromBack($left),
    } };
    ( $left:tt..=$right:tt ) => { $crate::SeqRangeInclusive {
        start: $crate::SeqIndex::FromFront($left),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( ^$left:tt..=$right:tt ) => { $crate::SeqRangeInclusive {
        start: $crate::SeqIndex::FromBack($left),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( $left:tt..=^$right:tt ) => { $crate::SeqRangeInclusive {
        start: $crate::SeqIndex::FromFront($left),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( ^$left:tt..=^$right:tt ) => { $crate::SeqRangeInclusive {
        start: $crate::SeqIndex::FromBack($left),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( ..$right:tt ) => { $crate::SeqRange {
        start: Default::default(),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( ..^$right:tt ) => { $crate::SeqRange {
        start: Default::default(),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( ..=$right:tt ) => { $crate::SeqRangeInclusive {
        start: Default::default(),
        end: $crate::SeqIndex::FromFront($right),
    } };
    ( ..=^$right:tt ) => { $crate::SeqRangeInclusive {
        start: Default::default(),
        end: $crate::SeqIndex::FromBack($right),
    } };
    ( ^$x:expr ) => { $crate::SeqIndex::FromBack($x) };
    ( $x:expr ) => { $crate::SeqIndex::FromFront($x) };
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
