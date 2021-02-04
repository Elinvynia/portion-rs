// Helpers used in crate code.

use crate::impls::Item;
use crate::interval::IntervalType::*;
use crate::Interval;

impl<T: Item> Interval<T> {
    pub(crate) fn singleton(&self) -> bool {
        self.itype == Singleton
    }

    pub(crate) fn left_open(&self) -> bool {
        matches!(self.itype, Open | OpenClosed)
    }

    pub(crate) fn left_closed(&self) -> bool {
        matches!(self.itype, Closed | ClosedOpen)
    }

    pub(crate) fn right_open(&self) -> bool {
        matches!(self.itype, Open | ClosedOpen)
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
                return LeftBound::Closed(self.lower());
            }
            return LeftBound::Open(other.lower.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if other.lower > self.lower {
                return LeftBound::Closed(other.lower.unwrap());
            }
            return LeftBound::Open(self.lower());
        }

        LeftBound::None
    }

    // Gets the common right point of two intervals.
    pub(crate) fn get_right_bound(&self, other: &Interval<T>) -> RightBound<T> {
        if self.right_open() && other.right_open() {
            let val = (self.upper).min(other.upper);
            return RightBound::Open(val.unwrap());
        }

        if self.right_closed() && other.right_closed() {
            let val = (self.upper).min(other.upper);
            return RightBound::Closed(val.unwrap());
        }

        if self.right_closed() && other.right_open() {
            if self.upper < other.upper {
                return RightBound::Open(self.upper());
            }
            return RightBound::Closed(other.upper.unwrap());
        }

        if self.right_open() && other.right_closed() {
            if other.upper < self.upper {
                return RightBound::Open(other.upper.unwrap());
            }
            return RightBound::Closed(self.upper());
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
                return LeftBound::Closed(self.lower());
            }
            return LeftBound::Open(other.lower.unwrap());
        }

        if self.left_open() && other.left_closed() {
            if other.lower < self.lower {
                return LeftBound::Closed(other.lower.unwrap());
            }
            return LeftBound::Open(self.lower());
        }

        if self.singleton() && other.singleton() {
            let val = (self.lower).min(other.lower);
            return LeftBound::Closed(val.unwrap());
        }

        if self.singleton() {
            if self.lower <= other.lower {
                return LeftBound::Closed(self.lower());
            }
            if other.left_closed() {
                return LeftBound::Closed(other.lower.unwrap());
            }
            return LeftBound::Open(other.lower.unwrap());
        }

        if other.singleton() {
            if other.lower <= self.lower {
                return LeftBound::Closed(other.lower.unwrap());
            }
            if self.left_closed() {
                return LeftBound::Closed(self.lower());
            }
            return LeftBound::Open(self.lower());
        }

        LeftBound::None
    }

    // Gets the highest right point of two intervals.
    pub(crate) fn get_right_val(&self, other: &Interval<T>) -> RightBound<T> {
        if self.right_open() && other.right_open() {
            let val = (self.upper).max(other.upper);
            return RightBound::Open(val.unwrap());
        }

        if self.right_closed() && other.right_closed() {
            let val = (self.upper).max(other.upper);
            return RightBound::Closed(val.unwrap());
        }

        if self.right_closed() && other.right_open() {
            if self.upper >= other.upper {
                return RightBound::Closed(self.upper());
            }
            return RightBound::Open(other.upper.unwrap());
        }

        if self.right_open() && other.right_closed() {
            if self.upper > other.upper {
                return RightBound::Open(self.upper());
            }
            return RightBound::Closed(other.upper.unwrap());
        }

        if self.singleton() && other.singleton() {
            let val = (self.lower).max(other.lower);
            return RightBound::Closed(val.unwrap());
        }

        if self.singleton() {
            if self.lower >= other.upper {
                return RightBound::Closed(self.lower());
            }
            if other.right_closed() {
                return RightBound::Closed(other.upper.unwrap());
            }
            return RightBound::Open(other.upper.unwrap());
        }

        if other.singleton() {
            if other.lower >= self.upper {
                return RightBound::Closed(other.lower.unwrap());
            }
            if self.right_closed() {
                return RightBound::Closed(self.upper());
            }
            return RightBound::Open(self.upper());
        }

        RightBound::None
    }
}

pub(crate) enum LeftBound<T: Item> {
    Open(T),
    Closed(T),
    None,
}

pub(crate) enum RightBound<T: Item> {
    Open(T),
    Closed(T),
    None,
}
