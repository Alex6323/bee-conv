//!  Meaningful type aliases and a signed 129 bit integer to store values at least up to (3^81-1)/2.

use std::ops::AddAssign;

/// Representation of an unsigned byte.
pub type Byte = u8;

/// Representation of a trit.
pub type Trit = i8;

/// Representation of a tryte.
pub type Tryte = u8;

/// A Sign for the `S129` signed 129 bit integer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Sign {
    /// Positive sign.
    Pos,
    /// Negative sign.
    Neg,
}

/// A signed 129 bit integer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct S129(pub Sign, pub u128);

impl AddAssign for S129 {
    fn add_assign(&mut self, other: Self) {
        match (&self.0, other.0) {
            (Sign::Pos, Sign::Neg) => {
                if other.1 > self.1 {
                    self.0 = Sign::Neg;
                    self.1 = other.1 - self.1;
                } else {
                    self.1 -= other.1;
                }
            }
            (Sign::Neg, Sign::Pos) => {
                if other.1 >= self.1 {
                    self.0 = Sign::Pos;
                    self.1 = other.1 - self.1;
                } else {
                    self.1 -= other.1;
                }
            }
            (_, _) => self.1 += other.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s129_add_assign() {
        // Example: (+, 2) + (-, 4) = (-, 2)
        let mut a = S129(Sign::Pos, 2);
        let b = S129(Sign::Neg, 4);
        a += b;
        assert_eq!(S129(Sign::Neg, 2), a);

        // Example: (+, 2) + (-, 1) = (+, 1)
        let mut a = S129(Sign::Pos, 2);
        let b = S129(Sign::Neg, 1);
        a += b;
        assert_eq!(S129(Sign::Pos, 1), a);

        // Example: (+, 2) + (-, 2) = (+, 0)
        let mut a = S129(Sign::Pos, 2);
        let b = S129(Sign::Neg, 2);
        a += b;
        assert_eq!(S129(Sign::Pos, 0), a);

        // Example: (-, 2) + (+, 4) = (+, 2)
        let mut a = S129(Sign::Neg, 2);
        let b = S129(Sign::Pos, 4);
        a += b;
        assert_eq!(S129(Sign::Pos, 2), a);

        // Example: (-, 2) + (+, 1) = (-, 1)
        let mut a = S129(Sign::Neg, 2);
        let b = S129(Sign::Pos, 1);
        a += b;
        assert_eq!(S129(Sign::Neg, 1), a);

        // Example: (-, 2) + (+, 2) = (+, 0)
        let mut a = S129(Sign::Neg, 2);
        let b = S129(Sign::Pos, 2);
        a += b;
        assert_eq!(S129(Sign::Pos, 0), a);

        // Example: (+, 2) + (+, 4) = (+, 6)
        let mut a = S129(Sign::Pos, 2);
        let b = S129(Sign::Pos, 4);
        a += b;
        assert_eq!(S129(Sign::Pos, 6), a);

        // Example: (-, 2) + (-, 4) = (-, 6)
        let mut a = S129(Sign::Neg, 2);
        let b = S129(Sign::Neg, 4);
        a += b;
        assert_eq!(S129(Sign::Neg, 6), a);
    }
}
