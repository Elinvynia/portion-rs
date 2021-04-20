use crate::{Interval, IntervalType, Item};

/// Iterator over values of the interval.
pub struct IntoIter<T: Item> {
    pub(crate) interval: Interval<T>,
    pub(crate) current: Option<T>,
}

impl<T: Item> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle trivial cases
        match self.interval.itype {
            IntervalType::Empty => return None,
            IntervalType::Singleton => return self.current.take(),
            _ => {}
        }

        if self.interval.right_open() && self.current >= self.interval.upper {
            return None;
        }

        if self.interval.right_closed() && self.current > self.interval.upper {
            return None;
        }

        let value = self.current;
        self.current = Some(self.current.unwrap().next());
        value
    }
}
