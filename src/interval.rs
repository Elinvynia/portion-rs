//! Holds the interval type and associated methods.

#![allow(clippy::suspicious_operation_groupings)]

use crate::Portion;
use std::fmt::Display;
use std::ops::BitAnd;

/// The interval type, main type of this library.
#[derive(Debug)]
pub struct Interval<T: Sized + PartialOrd + Clone + Display> {
    pub(crate) lower: Option<T>,
    pub(crate) upper: Option<T>,
    pub(crate) itype: IntervalType,
}

impl<T: Sized + PartialOrd + Clone + Display> Interval<T> {
    /// Returns the intersection of two intervals, shorthand for `interval & interval`.
    pub fn intersection(self, other: Interval<T>) -> Interval<T> {
        self & other
    }
}

impl<T: Sized + PartialOrd + Clone + Display> Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.empty() {
            return write!(f, "()");
        };
        use IntervalType::*;
        match self.itype {
            Open => write!(
                f,
                "({}, {})",
                self.lower.as_ref().unwrap(),
                self.upper.as_ref().unwrap()
            ),
            Closed => write!(
                f,
                "[{}, {}]",
                self.lower.as_ref().unwrap(),
                self.upper.as_ref().unwrap()
            ),
            Empty => write!(f, "()"),
            Singleton => write!(f, "[{}]", self.lower.as_ref().unwrap()),
            OpenClosed => write!(
                f,
                "({}, {}]",
                self.lower.as_ref().unwrap(),
                self.upper.as_ref().unwrap()
            ),
            ClosedOpen => write!(
                f,
                "[{}, {})",
                self.lower.as_ref().unwrap(),
                self.upper.as_ref().unwrap()
            ),
        }
    }
}

impl<T: Sized + PartialOrd + Clone + Display> BitAnd for Interval<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self.empty() || rhs.empty() {
            return Portion::empty();
        }
        if self.upper < rhs.lower {
            return Portion::empty();
        }
        if self.singleton() {
            if self.lower >= rhs.lower && self.lower <= rhs.upper {
                return rhs;
            } else {
                return Portion::empty();
            }
        }
        if rhs.singleton() {
            if rhs.lower >= self.lower && rhs.lower <= self.upper {
                return self;
            } else {
                return Portion::empty();
            }
        }
        if self.left_closed() && rhs.right_closed() {
            return Portion::closed(self.lower.unwrap(), rhs.upper.unwrap());
        }
        if self.left_open() && rhs.right_open() {
            return Portion::open(self.lower.unwrap(), rhs.upper.unwrap());
        }

        Portion::empty()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum IntervalType {
    Open,
    Closed,
    Empty,
    Singleton,
    OpenClosed,
    ClosedOpen,
}

pub(crate) trait IntervalOps {
    fn empty(&self) -> bool {
        true
    }

    fn atomic(&self) -> bool {
        true
    }
}

impl<T: Sized + PartialOrd + Clone + Display> IntervalOps for Interval<T> {
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
}

impl<T: Sized + PartialOrd + Clone + Display> Iterator for Interval<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T: Sized + PartialOrd + Clone + Display> IntervalOps for Vec<Interval<T>> {
    fn empty(&self) -> bool {
        self.iter().all(|i| i.empty())
    }

    fn atomic(&self) -> bool {
        self.len() < 2
    }
}
