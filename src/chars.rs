use std::ops::{Add, Sub};

pub trait Shift {
    fn shift<T>(self, count: T) -> Self
    where
        T: Into<AsciiShift>;
}

impl Shift for char {
    fn shift<T>(self, count: T) -> char
    where
        T: Into<AsciiShift>,
    {
        if self.is_ascii_lowercase() {
            let shift: AsciiShift = count.into();
            let ascii = self as u8;
            let result = ascii + shift.0;
            let start = 'a' as u8;
            return ((result - start).rem_euclid(26) + start) as char;
        } else {
            return self;
        }
    }
}

#[derive(Debug)]
pub struct AsciiShift(u8);

pub fn calc_shift(start: char, end: char) -> u8 {
    debug_assert!(start.is_ascii_lowercase());
    debug_assert!(end.is_ascii_lowercase());
    let shift: AsciiShift = (end as u8 as i8 - start as u8 as i8).into();
    return shift.0;
}

impl Add<AsciiShift> for AsciiShift {
    type Output = Self;
    fn add(self, rhs: AsciiShift) -> Self::Output {
        return Self((self.0 + rhs.0).rem_euclid(26));
    }
}

impl Sub<AsciiShift> for AsciiShift {
    type Output = Self;
    fn sub(self, rhs: AsciiShift) -> Self::Output {
        return Self(((self.0 as i16 - rhs.0 as i16).rem_euclid(26)) as u8);
    }
}

macro_rules! impl_ops {
    ($type:ident) => {
        impl From<$type> for AsciiShift {
            fn from(val: $type) -> Self {
                Self(((val.rem_euclid(26)) as u8))
            }
        }
    };
}

impl_ops!(u8);
impl_ops!(u16);
impl_ops!(u32);
impl_ops!(u64);
impl_ops!(usize);
impl_ops!(i8);
impl_ops!(i16);
impl_ops!(i32);
impl_ops!(i64);
impl_ops!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!('a'.shift(3), 'd');
        assert_eq!('a'.shift(-1), 'z');
        assert_eq!('a'.shift(0), 'a');
        assert_eq!('a'.shift(0), 'a');
        assert_eq!('a'.shift(26), 'a');
        assert_eq!('a'.shift(-27), 'z');
        assert_eq!(calc_shift('a', 'd'), 3);
        assert_eq!(calc_shift('a', 'a'), 0);
        assert_eq!(calc_shift('a', 'z'), 25);
    }
}
