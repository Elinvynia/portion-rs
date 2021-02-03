//! Holds the interval type and associated methods.

#![allow(clippy::suspicious_operation_groupings)]

use crate::impls::ITrait;
use crate::ops::IntervalOps;
use crate::IntervalType;
use std::fmt::Display;

/// The interval type, main type of this library.
pub struct Interval<T: ITrait> {
    pub(crate) lower: Option<T>,
    pub(crate) upper: Option<T>,
    pub(crate) itype: IntervalType,
    pub(crate) current: Option<T>,
}

impl<T: ITrait> PartialEq for Interval<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<T: ITrait> Eq for Interval<T> {}

impl<T: ITrait> Display for Interval<T> {
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
