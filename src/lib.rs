//! # portion-rs
//!
//! A simple interval library inspired by Python's `portion`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

pub use interval::Interval;

use interval::IntervalType;
use std::fmt::Display;
use std::marker::PhantomData;

pub(crate) mod helpers;
pub mod interval;

/// Blank type used for interval creation.
#[derive(Debug)]
pub struct Portion<T: Sized + PartialOrd + Clone + Display> {
    data1: PhantomData<T>,
    data2: PhantomData<T>,
}

impl<T: Sized + PartialOrd + Clone + Display> Portion<T> {
    /// Creates an open interval.
    pub fn open(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::Open,
        }
    }

    /// Creates a closed interval.
    pub fn closed(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::Closed,
        }
    }

    /// Creates an empty interval.
    pub fn empty() -> Interval<T> {
        Interval {
            lower: None,
            upper: None,
            itype: IntervalType::Empty,
        }
    }

    /// Creates a singleton interval.
    pub fn singleton(value: T) -> Interval<T> {
        Interval {
            lower: Some(value),
            upper: None,
            itype: IntervalType::Singleton,
        }
    }

    /// Creates an open-closed interval.
    pub fn openclosed(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::OpenClosed,
        }
    }

    /// Creates a closed-open interval.
    pub fn closedopen(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::ClosedOpen,
        }
    }
}
