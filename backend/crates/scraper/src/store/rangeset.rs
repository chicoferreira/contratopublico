use std::ops::{Add, Sub};

use num_traits::One;
use serde::{Deserialize, Serialize};

/// A data structure that efficiently stores and manages sets of discrete, ordered values
/// as disjoint, sorted intervals (ranges). This allows compact representation of contiguous
/// sequences and efficient operations like insertion and lookup.
///
/// This is used to store the saved pages of the scraper efficiently.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RangeSet<T> {
    ranges: Vec<(T, T)>,
}

impl<T> RangeSet<T>
where
    T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy + One,
{
    pub fn new() -> Self {
        RangeSet::<T> { ranges: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        if self.ranges.is_empty() {
            self.ranges.push((value, value));
            return;
        }

        let one = T::one();
        // Find the insertion point: number of ranges where start <= value
        let pos = self.ranges.partition_point(|r| r.0 <= value);

        // Check if contained in previous range
        if pos > 0 {
            let prev_idx = pos - 1;
            let (prev_start, prev_end) = self.ranges[prev_idx];
            if prev_start <= value && value <= prev_end {
                return; // Already present
            }
        }

        // Determine if we can merge with prev and/or next
        let mut merge_with_prev = false;
        let mut merge_with_next = false;

        if pos > 0 {
            let prev_idx = pos - 1;
            let prev_end = self.ranges[prev_idx].1;
            if value == prev_end + one {
                merge_with_prev = true;
            }
        }

        if pos < self.ranges.len() {
            let next_start = self.ranges[pos].0;
            if value == next_start - one {
                merge_with_next = true;
            }
        }

        match (merge_with_prev, merge_with_next) {
            (true, true) => {
                // Merge prev, value, and next into prev
                let prev_idx = pos - 1;
                self.ranges[prev_idx].1 = self.ranges[pos].1;
                self.ranges.remove(pos);
            }
            (true, false) => {
                // Extend prev to include value
                let prev_idx = pos - 1;
                self.ranges[prev_idx].1 = value;
            }
            (false, true) => {
                // Extend next to include value
                self.ranges[pos].0 = value;
            }
            (false, false) => {
                // Insert new singleton range
                self.ranges.insert(pos, (value, value));
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.ranges
            .iter()
            .any(|&(start, end)| start <= *value && *value <= end)
    }

    pub fn get_first_missing(&self, min: T) -> T {
        for (start, end) in self.ranges.iter() {
            if *start <= min && min <= *end {
                return *end + T::one();
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut set = RangeSet::<i32>::new();
        set.insert(1);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
    }

    #[test]
    fn test_get_first_missing() {
        let mut set = RangeSet::<i32>::new();
        assert_eq!(set.get_first_missing(0), 0);
        set.insert(1);
        set.insert(3);
        assert_eq!(set.get_first_missing(0), 0);
        assert_eq!(set.get_first_missing(1), 2);
        assert_eq!(set.get_first_missing(2), 2);
        assert_eq!(set.get_first_missing(3), 4);
        assert_eq!(set.get_first_missing(4), 4);
    }

    #[test]
    fn test_internal_struct() {
        let mut set = RangeSet::<i32>::new();
        set.insert(1);
        assert_eq!(set.ranges, vec![(1, 1)]);
        set.insert(5);
        assert_eq!(set.ranges, vec![(1, 1), (5, 5)]);
        set.insert(3);
        assert_eq!(set.ranges, vec![(1, 1), (3, 3), (5, 5)]);
        set.insert(2);
        assert_eq!(set.ranges, vec![(1, 3), (5, 5)]);
        set.insert(4);
        assert_eq!(set.ranges, vec![(1, 5)]);
        set.insert(6);
        assert_eq!(set.ranges, vec![(1, 6)]);
        set.insert(8);
        assert_eq!(set.ranges, vec![(1, 6), (8, 8)]);
        set.insert(9);
        assert_eq!(set.ranges, vec![(1, 6), (8, 9)]);
        set.insert(0);
        assert_eq!(set.ranges, vec![(0, 6), (8, 9)]);
    }
}
