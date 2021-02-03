// Helpers used in crate code.

use crate::impls::ITrait;
use crate::{Interval, IntervalType::*};

impl<T: ITrait> Interval<T> {
    pub(crate) fn singleton(&self) -> bool {
        self.itype == Singleton
    }

    pub(crate) fn left_open(&self) -> bool {
        matches!(self.itype, Open | OpenClosed)
    }

    pub(crate) fn left_closed(&self) -> bool {
        matches!(self.itype, Closed | ClosedOpen)
    }

    pub(crate) fn right_closed(&self) -> bool {
        matches!(self.itype, Closed | OpenClosed)
    }

    pub(crate) fn contains(self, thing: T) -> bool {
        self.into_iter().any(|x| x == thing)
    }

    // Gets the common left point of two intervals.
    pub(crate) fn get_left_bound(&self, other: &Interval<T>) -> LeftBound<T> {
        if self.left_open() && other.left_open() {
            let val = (self.lower).max(other.lower);
            return LeftBound::Open(val.unwrap());
        }

        if self.left_closed() && other.left_closed() {
            let val = (self.lower).max(other.lower);
            return LeftBound::Closed(val.unwrap());
        }

        if self.left_closed() && other.left_open() {
            if self.lower > other.lower {
                return LeftBound::Closed(self.lower.unwrap());
            }
            return LeftBound::Open(other.lower.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if other.lower > self.lower {
                return LeftBound::Closed(other.lower.unwrap());
            }
            return LeftBound::Open(self.lower.unwrap());
        }

        LeftBound::None
    }

    // Gets the common right point of two intervals.
    pub(crate) fn get_right_bound(&self, other: &Interval<T>) -> RightBound<T> {
        if self.left_open() && other.left_open() {
            let val = (self.upper).min(other.upper);
            return RightBound::Open(val.unwrap());
        }

        if self.left_closed() && other.left_closed() {
            let val = (self.upper).min(other.upper);
            return RightBound::Closed(val.unwrap());
        }

        if self.left_closed() && other.left_open() {
            if self.upper < other.upper {
                return RightBound::Open(self.upper.unwrap());
            }
            return RightBound::Closed(other.upper.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if other.upper < self.upper {
                return RightBound::Open(other.upper.unwrap());
            }
            return RightBound::Closed(self.upper.unwrap());
        }

        RightBound::None
    }

    // Gets the lowest left point of two intervals.
    pub(crate) fn get_left_val(&self, other: &Interval<T>) -> LeftBound<T> {
        if self.left_open() && other.left_open() {
            let val = (self.lower).min(other.lower);
            return LeftBound::Open(val.unwrap());
        }

        if self.left_closed() && other.left_closed() {
            let val = (self.lower).min(other.lower);
            return LeftBound::Closed(val.unwrap());
        }

        if self.left_closed() && other.left_open() {
            if self.lower < other.lower {
                return LeftBound::Closed(self.lower.unwrap());
            }
            return LeftBound::Open(other.lower.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if other.lower < self.lower {
                return LeftBound::Closed(other.lower.unwrap());
            }
            return LeftBound::Open(self.lower.unwrap());
        }

        LeftBound::None
    }

    // Gets the highest right point of two intervals.
    pub(crate) fn get_right_val(&self, other: &Interval<T>) -> RightBound<T> {
        if self.left_open() && other.left_open() {
            let val = (self.upper).max(other.upper);
            return RightBound::Open(val.unwrap());
        }

        if self.left_closed() && other.left_closed() {
            let val = (self.upper).max(other.upper);
            return RightBound::Closed(val.unwrap());
        }

        if self.left_closed() && other.left_open() {
            if self.upper >= other.upper {
                return RightBound::Closed(self.upper.unwrap());
            }
            return RightBound::Open(other.upper.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if self.upper > other.upper {
                return RightBound::Open(self.upper.unwrap());
            }
            return RightBound::Closed(other.upper.unwrap());
        }

        RightBound::None
    }
}

pub(crate) enum LeftBound<T: ITrait> {
    Open(T),
    Closed(T),
    None,
}

pub(crate) enum RightBound<T: ITrait> {
    Open(T),
    Closed(T),
    None,
}
