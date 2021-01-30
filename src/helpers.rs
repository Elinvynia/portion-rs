use crate::{Interval, IntervalType::*};
use std::fmt::Display;

impl<T: Sized + PartialOrd + Clone + Display> Interval<T> {
    pub(crate) fn left_open(&self) -> bool {
        matches!(self.itype, Open | OpenClosed)
    }

    pub(crate) fn right_open(&self) -> bool {
        matches!(self.itype, Open | ClosedOpen)
    }

    pub(crate) fn left_closed(&self) -> bool {
        matches!(self.itype, Closed | ClosedOpen)
    }

    pub(crate) fn right_closed(&self) -> bool {
        matches!(self.itype, Closed | OpenClosed)
    }

    pub(crate) fn singleton(&self) -> bool {
        self.itype == Singleton
    }
}
