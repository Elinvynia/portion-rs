//! Holds the interval type and associated methods.

use crate::impls::Item;
use crate::iter::IntoIter;
use crate::ops::Operations;
use std::fmt::Display;

/// The interval type, main type of this library.
pub struct Interval<T: Item> {
    pub(crate) lower: Option<T>,
    pub(crate) upper: Option<T>,
    pub(crate) itype: IntervalType,
}

impl<T: Item> Interval<T> {
    /// Iterator over references of this interval.
    pub fn iter(&self) {
        todo!()
    }

    /// Iterator over mutable references of this interval.
    pub fn iter_mut(&mut self) {
        todo!()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum IntervalType {
    Open,
    Closed,
    Empty,
    Singleton,
    OpenClosed,
    ClosedOpen,
}

impl<T: Item> PartialEq for Interval<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<T: Item> Eq for Interval<T> {}

impl<T: Item> Display for Interval<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.empty() {
            return write!(f, "()");
        };
        use IntervalType::*;
        match self.itype {
            Open => write!(f, "({}, {})", self.lower(), self.upper()),
            Closed => write!(f, "[{}, {}]", self.lower(), self.upper()),
            Empty => write!(f, "()"),
            Singleton => write!(f, "[{}]", self.lower()),
            OpenClosed => write!(f, "({}, {}]", self.lower(), self.upper()),
            ClosedOpen => write!(f, "[{}, {})", self.lower(), self.upper()),
        }
    }
}

impl<T: Item> IntoIterator for Interval<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        use IntervalType::*;
        let current = match self.itype {
            Open | OpenClosed => Some(self.lower().next()),
            _ => self.lower,
        };

        IntoIter {
            current,
            interval: self,
        }
    }
}
