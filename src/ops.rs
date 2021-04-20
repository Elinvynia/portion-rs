//! Operations defined on intervals.

use crate::helpers::{LeftBound, RightBound};
use crate::impls::Item;
use crate::{Interval, IntervalType, Portion};
use std::ops::{BitAnd, BitOr};

/// Operations defined on interval-like things.
pub trait Operations: Sized {
    /// The return type used in implementations.
    type Output;

    /// Returns whether the interval is empty, regardless of it's actual type.
    fn empty(&self) -> bool {
        true
    }

    /// Returns whether the interval is atomic.
    fn atomic(&self) -> bool {
        true
    }

    /// Returns the intersection of two intervals, shorthand for `interval & interval`.
    fn intersection(self, _other: Self::Output) -> Self::Output {
        unimplemented!()
    }

    /// Returns the union of two intervals, shorthand for `interval | interval`.
    fn union(self, _other: Self::Output) -> Self::Output {
        unimplemented!()
    }
}

impl<T: Item> Operations for Interval<T> {
    type Output = Self;

    fn empty(&self) -> bool {
        use IntervalType::*;
        match self.itype {
            Open => self.lower >= self.upper,
            Closed => self.lower > self.upper,
            Empty => true,
            Singleton => false,
            OpenClosed => self.lower >= self.upper,
            ClosedOpen => self.lower >= self.upper,
        }
    }

    fn intersection(self, other: Self::Output) -> Self::Output {
        self & other
    }

    fn union(self, other: Self::Output) -> Self::Output {
        self | other
    }
}

// Intersection.
impl<T: Item> BitAnd for Interval<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        // Optimization.
        if self.empty() || rhs.empty() {
            return Portion::empty();
        }

        // Handle singletons first.
        if self.singleton() {
            if rhs.contains(self.lower()) {
                return self;
            } else {
                return Portion::empty();
            }
        }

        if rhs.singleton() {
            if self.contains(rhs.lower.unwrap()) {
                return rhs;
            } else {
                return Portion::empty();
            }
        }

        // Optimization.
        if self.upper < rhs.lower {
            return Portion::empty();
        }

        // Match the bounds.
        let left_bound = self.get_left_bound(&rhs);
        let right_bound = self.get_right_bound(&rhs);

        match left_bound {
            LeftBound::Open(lower) => match right_bound {
                RightBound::Open(upper) => Portion::open(lower, upper),
                RightBound::Closed(upper) => Portion::openclosed(lower, upper),
                RightBound::None => Portion::empty(),
            },
            LeftBound::Closed(lower) => match right_bound {
                RightBound::Open(upper) => Portion::closedopen(lower, upper),
                RightBound::Closed(upper) => Portion::closed(lower, upper),
                RightBound::None => Portion::empty(),
            },
            LeftBound::None => Portion::empty(),
        }
    }
}

// Union.
impl<T: Item> BitOr for Interval<T> {
    type Output = Self;

    fn bitor(self, rhs: Interval<T>) -> Self::Output {
        if self.empty() {
            return rhs;
        }

        if rhs.empty() {
            return self;
        }

        // TODO: return a union of two intervals?
        #[allow(clippy::suspicious_operation_groupings)]
        if !self.singleton() && !rhs.singleton() && self.upper < rhs.lower {
            return Portion::empty();
        }

        let left_val = self.get_lowest_val(&rhs);
        let right_val = self.get_highest_val(&rhs);

        match left_val {
            LeftBound::Closed(lower) => match right_val {
                RightBound::Closed(upper) => Portion::closed(lower, upper),
                RightBound::Open(upper) => Portion::closedopen(lower, upper),
                RightBound::None => unreachable!(),
            },
            LeftBound::Open(lower) => match right_val {
                RightBound::Closed(upper) => Portion::openclosed(lower, upper),
                RightBound::Open(upper) => Portion::open(lower, upper),
                RightBound::None => unreachable!(),
            },
            LeftBound::None => unreachable!(),
        }
    }
}
