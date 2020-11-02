use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Portion;

impl Portion {
    pub fn open<T: Sized + PartialOrd + Display>(vec: Vec<T>) -> Open<T> {
        Open {vec}
    }
    pub fn closed<T: Sized + PartialOrd + Display>(vec: Vec<T>) -> Closed<T> {
        Closed {vec}
    }
}

trait Interval {
    fn empty(&self) -> bool {true}
    fn atomic() -> bool {true}
}

pub struct Open<T: Sized + PartialOrd + Display> {
    vec: Vec<T>,
}

impl<T: Sized + PartialOrd + Display> Display for Open<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = String::new();
        for x in self.vec.iter() {
            s += &x.to_string();
            s.push_str(", ")
        }

        s.truncate(s.len() - 1);

        write!(f, "({})", s)
    }
}

impl<T: Sized + PartialOrd + Display> Interval for Open<T> {
    fn empty(&self) -> bool {
        if self.vec.len() > 2 {
            return false
        };
        if self.vec.len() == 0 {
            return true
        };
        if self.vec.len() == 1 {
            return false
        };
        if self.vec.len() == 2 && self.vec[0] == self.vec[1] {
            return true
        };
        return false
    }

    fn atomic() -> bool {true}
}

pub struct Closed<T: Sized + PartialOrd + Display> {
    vec: Vec<T>
}

impl<T: Sized + PartialOrd + Display> Display for Closed<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = String::new();
        for x in self.vec.iter() {
            s += &x.to_string();
            s.push_str(", ")
        }

        s.truncate(s.len() - 1);

        write!(f, "[{}]", s)
    }
}

pub struct OpenClosed;

impl Display for OpenClosed {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(]")
    }
}


pub struct ClosedOpen;

impl Display for ClosedOpen {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[)")
    }
}
