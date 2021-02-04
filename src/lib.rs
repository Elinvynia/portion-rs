//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # portion-rs
//!
//! A simple interval library inspired by Python's `portion`.
//!
//! [ci]: https://github.com/Elinvynia/portion-rs/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/portion-rs/Rust/master?style=flat-square
//! [docs]: https://docs.rs/portion-rs
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/portion-rs
//! [crate-version]: https://img.shields.io/crates/v/portion-rs.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use interval::Interval;
pub use intervals::Intervals;
pub use ops::Operations;

use crate::impls::Item;
use crate::interval::IntervalType;
use std::marker::PhantomData;

pub(crate) mod helpers;
pub(crate) mod impls;
pub mod interval;
pub mod intervals;
pub mod ops;

/// Blank type used for interval creation.
pub struct Portion<T: Item> {
    data: PhantomData<T>,
}

impl<T: Item> Portion<T> {
    /// Creates an open interval.
    pub fn open(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::Open,
            current: None,
        }
    }

    /// Creates a closed interval.
    pub fn closed(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::Closed,
            current: None,
        }
    }

    /// Creates an empty interval.
    pub fn empty() -> Interval<T> {
        Interval {
            lower: None,
            upper: None,
            itype: IntervalType::Empty,
            current: None,
        }
    }

    /// Creates a singleton interval.
    pub fn singleton(value: T) -> Interval<T> {
        Interval {
            lower: Some(value),
            upper: None,
            itype: IntervalType::Singleton,
            current: None,
        }
    }

    /// Creates an open-closed interval.
    pub fn openclosed(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::OpenClosed,
            current: None,
        }
    }

    /// Creates a closed-open interval.
    pub fn closedopen(lower: T, upper: T) -> Interval<T> {
        Interval {
            lower: Some(lower),
            upper: Some(upper),
            itype: IntervalType::ClosedOpen,
            current: None,
        }
    }
}
