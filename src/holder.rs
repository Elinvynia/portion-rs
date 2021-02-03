//! Holds the type for a vector of intervals.

use crate::impls::ITrait;
use crate::interval::Interval;
use std::fmt::Display;

/// A vector of intervals.
#[derive(PartialEq, Eq)]
pub struct Intervals<T: ITrait> {
    pub(crate) data: Vec<Interval<T>>,
}

impl<T: ITrait> Intervals<T> {
    pub(crate) fn new(data: Vec<Interval<T>>) -> Self {
        Intervals { data }
    }
}

impl<T: ITrait> Display for Intervals<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for x in self.data.iter() {
            s += &x.to_string();
            if Some(x) != self.data.last() {
                s += " | ";
            }
        }

        write!(fmt, "{}", s)
    }
}
