use std::{fmt::Debug, hash::Hash};

/// This trait defines an auto-increment integer.
///
/// However, you can implement it for your own types.
pub trait Aictable: Clone + Debug + Eq + PartialEq + Hash {
    /// The initial value for the type implementing this trait.
    const INITIAL: Self;

    /// Checks if the maximum value for the type has been reached.
    fn is_max_reached(&self) -> bool;

    /// Returns the next value, saturating at the numeric bounds instead of overflowing.
    fn saturating_next(&self) -> Self;

    /// Returns the next value, wrapping around at the numeric bounds.
    fn wrapping_next(&self) -> Self;
}

macro_rules! impl_aictable {
    ($($t:ty),*) => {
        $(
            impl Aictable for $t {
                const INITIAL: Self = Self::MIN;

                fn is_max_reached(&self) -> bool {
                    *self == Self::MAX
                }

                fn saturating_next(&self) -> Self {
                    self.saturating_add(1)
                }

                fn wrapping_next(&self) -> Self {
                    self.wrapping_add(1)
                }
            }
        )*
    };
}

impl_aictable!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aictable_i8() {
        let mut num = i8::INITIAL;
        assert_eq!(num, i8::MIN);
        assert!(!num.is_max_reached());

        num = i8::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), i8::MAX);
        assert_eq!(num.wrapping_next(), i8::MIN);
    }

    #[test]
    fn test_aictable_i16() {
        let mut num = i16::INITIAL;
        assert_eq!(num, i16::MIN);
        assert!(!num.is_max_reached());

        num = i16::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), i16::MAX);
        assert_eq!(num.wrapping_next(), i16::MIN);
    }

    #[test]
    fn test_aictable_i32() {
        let mut num = i32::INITIAL;
        assert_eq!(num, i32::MIN);
        assert!(!num.is_max_reached());

        num = i32::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), i32::MAX);
        assert_eq!(num.wrapping_next(), i32::MIN);
    }

    #[test]
    fn test_aictable_i64() {
        let mut num = i64::INITIAL;
        assert_eq!(num, i64::MIN);
        assert!(!num.is_max_reached());

        num = i64::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), i64::MAX);
        assert_eq!(num.wrapping_next(), i64::MIN);
    }

    #[test]
    fn test_aictable_isize() {
        let mut num = isize::INITIAL;
        assert_eq!(num, isize::MIN);
        assert!(!num.is_max_reached());

        num = isize::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), isize::MAX);
        assert_eq!(num.wrapping_next(), isize::MIN);
    }

    #[test]
    fn test_aictable_u8() {
        let mut num = u8::INITIAL;
        assert_eq!(num, u8::MIN);
        assert!(!num.is_max_reached());

        num = u8::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), u8::MAX);
        assert_eq!(num.wrapping_next(), u8::MIN);
    }

    #[test]
    fn test_aictable_u16() {
        let mut num = u16::INITIAL;
        assert_eq!(num, u16::MIN);
        assert!(!num.is_max_reached());

        num = u16::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), u16::MAX);
        assert_eq!(num.wrapping_next(), u16::MIN);
    }

    #[test]
    fn test_aictable_u32() {
        let mut num = u32::INITIAL;
        assert_eq!(num, u32::MIN);
        assert!(!num.is_max_reached());

        num = u32::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), u32::MAX);
        assert_eq!(num.wrapping_next(), u32::MIN);
    }

    #[test]
    fn test_aictable_u64() {
        let mut num = u64::INITIAL;
        assert_eq!(num, u64::MIN);
        assert!(!num.is_max_reached());

        num = u64::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), u64::MAX);
        assert_eq!(num.wrapping_next(), u64::MIN);
    }

    #[test]
    fn test_aictable_usize() {
        let mut num = usize::INITIAL;
        assert_eq!(num, usize::MIN);
        assert!(!num.is_max_reached());

        num = usize::MAX;
        assert!(num.is_max_reached());
        assert_eq!(num.saturating_next(), usize::MAX);
        assert_eq!(num.wrapping_next(), usize::MIN);
    }
}
