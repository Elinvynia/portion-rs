use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Portion;

impl Portion {
    pub fn open<T: Sized + PartialOrd + Display>(lower: T, upper: T) -> Open<T> {
        Open { lower, upper }
    }
    pub fn closed<T: Sized + PartialOrd + Display>(lower: T, upper: T) -> Closed<T> {
        Closed { lower, upper }
    }
    pub fn empty() -> Empty {
        Empty
    }
    pub fn singleton<T: Sized + PartialOrd + Display>(value: T) -> Singleton<T> {
        Singleton { value }
    }
    pub fn openclosed<T: Sized + PartialOrd + Display>(lower: T, upper: T) -> OpenClosed<T> {
        OpenClosed { lower, upper }
    }
    pub fn closedopen<T: Sized + PartialOrd + Display>(lower: T, upper: T) -> ClosedOpen<T> {
        ClosedOpen { lower, upper }
    }
}

trait Interval {
    fn empty(&self) -> bool {
        true
    }
    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::Empty
    }
}

pub enum IntervalType {
    Open,
    Closed,
    Empty,
    Singleton,
    OpenClosed,
    ClosedOpen,
}

pub struct Open<T: Sized + PartialOrd + Display> {
    lower: T,
    upper: T,
}

impl<T: Sized + PartialOrd + Display> Display for Open<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.empty() {
            return write!(f, "()");
        }

        write!(f, "({}, {})", self.lower, self.upper)
    }
}

impl<T: Sized + PartialOrd + Display> Interval for Open<T> {
    fn empty(&self) -> bool {
        self.lower >= self.upper
    }

    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::Open
    }
}

pub struct Closed<T: Sized + PartialOrd + Display> {
    lower: T,
    upper: T,
}

impl<T: Sized + PartialOrd + Display> Display for Closed<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.empty() {
            return write!(f, "()");
        }

        if self.lower == self.upper {
            write!(f, "[{}]", self.lower)
        } else {
            write!(f, "[{}, {}]", self.lower, self.upper)
        }
    }
}

impl<T: Sized + PartialOrd + Display> Interval for Closed<T> {
    fn empty(&self) -> bool {
        self.lower > self.upper
    }

    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::Closed
    }
}

pub struct Empty;

impl Interval for Empty {}

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "()")
    }
}

pub struct Singleton<T: Sized + PartialOrd + Display> {
    value: T,
}

impl<T: Sized + PartialOrd + Display> Interval for Singleton<T> {
    fn empty(&self) -> bool {
        false
    }
    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::Singleton
    }
}

impl<T: Sized + PartialOrd + Display> Display for Singleton<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[{}]", self.value)
    }
}

pub struct OpenClosed<T: Sized + PartialOrd + Display> {
    lower: T,
    upper: T,
}

impl<T: Sized + PartialOrd + Display> Display for OpenClosed<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.empty() {
            return write!(f, "(]");
        }
        write!(f, "({}, {}]", self.lower, self.upper)
    }
}

impl<T: Sized + PartialOrd + Display> Interval for OpenClosed<T> {
    fn empty(&self) -> bool {
        self.lower >= self.upper
    }

    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::OpenClosed
    }
}

pub struct ClosedOpen<T: Sized + PartialOrd + Display> {
    lower: T,
    upper: T,
}

impl<T: Sized + PartialOrd + Display> Display for ClosedOpen<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.empty() {
            return write!(f, "[)");
        }
        write!(f, "[{}, {})", self.lower, self.upper)
    }
}

impl<T: Sized + PartialOrd + Display> Interval for ClosedOpen<T> {
    fn empty(&self) -> bool {
        self.lower >= self.upper
    }
    fn atomic(&self) -> bool {
        true
    }
    fn itype(&self) -> IntervalType {
        IntervalType::ClosedOpen
    }
}
