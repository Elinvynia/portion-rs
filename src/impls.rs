use std::fmt::Display;

pub trait Item: Sized + Ord + Copy + Display {
    fn next(self) -> Self {
        unimplemented!()
    }

    fn next_ref(self) -> Self {
        unimplemented!()
    }

    fn prev(self) -> Self {
        unimplemented!()
    }

    fn minimum(self) -> Self {
        unimplemented!()
    }

    fn maximum(self) -> Self {
        unimplemented!()
    }
}

macro_rules! impl_num {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn next(self) -> Self {
                self + 1
            }

            fn prev(self) -> Self {
                self - 1
            }

            fn minimum(self) -> Self {
                <$t>::MIN
            }

            fn maximum(self) -> Self {
                <$t>::MAX
            }
        }
    )*)
}

impl_num!(Item for usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
