//! Operations defined on intervals.

use crate::helpers::{LeftBound, RightBound};
use crate::impls::Item;
use crate::intervals::Intervals;
use crate::{Interval, IntervalType, Portion};
use std::ops::{BitAnd, BitOr, Neg};

/// Operations defined on intervals.
pub trait Operations: Sized {
    /// The interval type used in implementations.
    type Single;

    /// Type for returning multiple intervals.
    type Multiple;

    /// Returns whether the interval(s) is empty, regardless of it's actual type.
    fn empty(&self) -> bool {
        true
    }

    /// Returns whether the interval(s) is atomic.
    fn atomic(&self) -> bool {
        true
    }

    /// Returns the intersection of two intervals, shorthand for `interval & interval`.
    fn intersection(self, _other: Self::Single) -> Self::Single {
        unimplemented!()
    }

    /// Returns the union of two intervals, shorthand for `interval | interval`.
    fn union(self, _other: Self::Single) -> Self::Single {
        unimplemented!()
    }

    /// Returns the complement of the interval, shorthand for `-interval`
    fn complement(self) -> Self::Multiple {
        unimplemented!()
    }
}

impl<T: Item> Operations for Interval<T> {
    type Single = Self;
    type Multiple = Intervals<T>;

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

    fn intersection(self, other: Self::Single) -> Self::Single {
        self & other
    }

    fn union(self, other: Self::Single) -> Self::Single {
        self | other
    }

    fn complement(self) -> Self::Multiple {
        -self
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

        let left_val = self.get_left_val(&rhs);
        let right_val = self.get_right_val(&rhs);

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

// Complement.
impl<T: Item> Neg for Interval<T> {
    type Output = Intervals<T>;

    fn neg(self) -> Self::Output {
        let left_lower = self.lower().minimum();
        let left_upper = self.lower();
        let left;
        if self.left_closed() {
            left = Portion::closedopen(left_lower, left_upper);
        } else {
            left = Portion::closed(left_lower, left_upper);
        }

        let right_lower = self.upper();
        let right_upper = self.upper().maximum();
        let right;
        if self.right_closed() {
            right = Portion::openclosed(right_lower, right_upper);
        } else {
            right = Portion::closed(right_lower, right_upper)
        }

        Intervals::new(vec![left, right])
    }
}
