//! This module contains the bit datatype with gates.

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// The most primitive datatype, all other data types use this datatype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    /// Logical 0, boolean false
    Low,

    /// Logical 1, boolean true
    High,
}

impl Bit {
    /// The most primitive gate of this library, all other gates and circuits use this
    pub const fn nand(self, other: Self) -> Self {
        if matches!((self, other), (Self::High, Self::High)) {
            Self::Low
        } else {
            Self::High
        }
    }

    /// Inverts the bit
    pub const fn not(self) -> Self {
        self.nand(self)
    }

    /// Returns `Bit::High` if both bits are high
    pub const fn and(self, other: Self) -> Self {
        self.nand(other).not()
    }

    /// Returns `Bit::High` if either or both bits are high
    pub const fn or(self, other: Self) -> Self {
        self.not().nand(other.not())
    }

    /// Returns `Bit::High` if neither bit is heigh
    pub const fn nor(self, other: Self) -> Self {
        self.or(other).not()
    }

    /// Returns `Bit::High` if none or both bits are high
    pub const fn xnor(self, other: Self) -> Self {
        self.nand(other).nand(self.or(other))
    }

    /// Returns `Bit::High` if either but not both bits are high
    pub const fn xor(self, other: Self) -> Self {
        self.xnor(other).not()
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value {
            Self::High
        } else {
            Self::Low
        }
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        matches!(value, Bit::High)
    }
}

impl BitAnd for Bit {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.and(rhs);
    }
}

impl BitOr for Bit {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.or(rhs);
    }
}

impl BitXor for Bit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.xor(rhs);
    }
}

impl Not for Bit {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

#[cfg(test)]
mod tests {
    use super::Bit;

    #[test]
    fn nand() {
        assert!(bool::from(Bit::Low.nand(Bit::Low)));
        assert!(bool::from(Bit::Low.nand(Bit::High)));
        assert!(bool::from(Bit::High.nand(Bit::Low)));
        assert!(!bool::from(Bit::High.nand(Bit::High)));
    }

    #[test]
    fn not() {
        assert!(bool::from(Bit::Low.not()));
        assert!(!bool::from(Bit::High.not()));
    }

    #[test]
    fn and() {
        assert!(!bool::from(Bit::Low.and(Bit::Low)));
        assert!(!bool::from(Bit::Low.and(Bit::High)));
        assert!(!bool::from(Bit::High.and(Bit::Low)));
        assert!(bool::from(Bit::High.and(Bit::High)));
    }

    #[test]
    fn or() {
        assert!(!bool::from(Bit::Low.or(Bit::Low)));
        assert!(bool::from(Bit::Low.or(Bit::High)));
        assert!(bool::from(Bit::High.or(Bit::Low)));
        assert!(bool::from(Bit::High.or(Bit::High)));
    }

    #[test]
    fn nor() {
        assert!(bool::from(Bit::Low.nor(Bit::Low)));
        assert!(!bool::from(Bit::Low.nor(Bit::High)));
        assert!(!bool::from(Bit::High.nor(Bit::Low)));
        assert!(!bool::from(Bit::High.nor(Bit::High)));
    }

    #[test]
    fn xor() {
        assert!(!bool::from(Bit::Low.xor(Bit::Low)));
        assert!(bool::from(Bit::Low.xor(Bit::High)));
        assert!(bool::from(Bit::High.xor(Bit::Low)));
        assert!(!bool::from(Bit::High.xor(Bit::High)));
    }

    #[test]
    fn xnor() {
        assert!(bool::from(Bit::Low.xnor(Bit::Low)));
        assert!(!bool::from(Bit::Low.xnor(Bit::High)));
        assert!(!bool::from(Bit::High.xnor(Bit::Low)));
        assert!(bool::from(Bit::High.xnor(Bit::High)));
    }
}
