//! Holds the interval type and associated methods.

use crate::impls::Item;
use crate::ops::Operations;
use std::fmt::Display;

/// The interval type, main type of this library.
pub struct Interval<T: Item> {
    pub(crate) lower: Option<T>,
    pub(crate) upper: Option<T>,
    pub(crate) itype: IntervalType,
    pub(crate) current: Option<T>,
}

impl<T: Item> Interval<T> {
    pub(crate) fn lower(&self) -> T {
        self.lower.unwrap()
    }

    pub(crate) fn upper(&self) -> T {
        self.upper.unwrap()
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

impl<T: Item> Iterator for Interval<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty() {
            return None;
        }
        if self.singleton() {
            return self.lower.take();
        }

        if self.current.is_none() {
            if self.left_closed() {
                self.current = Some(self.lower());
                return Some(self.lower());
            }

            self.current = Some(self.lower().next());
            return Some(self.lower().next());
        }

        let current = self.current.unwrap();
        let next = Some(current.next());
        if next < self.upper {
            self.current = next;
            return next;
        }
        if next == self.upper && self.right_closed() {
            self.current = next;
            return next;
        }
        None
    }
}
