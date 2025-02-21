//! Muxes interacting on the bytes.

use core::{
    array,
    ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub},
};

use crate::bit::Bit;

/// The byte datatype is the smallest datatype a pointer can point to
#[derive(Debug, Clone, Copy)]
pub struct Byte {
    bits: [Bit; 8],
}

impl Byte {
    /// Applies the `Bit::nand` function on the bits of both bytes
    pub fn nand(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].nand(other.bits[index])),
        }
    }

    /// Applies the `Bit::and` function on the bits of both bytes
    pub fn and(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].and(other.bits[index])),
        }
    }

    /// Applies the `Bit::or` function on the bits of both bytes
    pub fn or(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].or(other.bits[index])),
        }
    }

    /// Applies the `Bit::nor` function on the bits of both bytes
    pub fn nor(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].nor(other.bits[index])),
        }
    }

    /// Applies the `Bit::xnor` function on the bits of both bytes
    pub fn xnor(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].xnor(other.bits[index])),
        }
    }

    /// Applies the `Bit::xor` function on the bits of both bytes
    pub fn xor(&self, other: &Self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].xor(other.bits[index])),
        }
    }

    /// Adds 2 bytes and the carry bit
    pub fn add_with_carry(self, right: Self, mut carry: Bit) -> (Self, Bit) {
        let left: [Bit; 8] = self.into();
        let right: [Bit; 8] = right.into();
        (
            Self::from(array::from_fn(|i| {
                let result;
                (result, carry) = left[i].full_adder(right[i], carry);
                result
            })),
            carry,
        )
    }

    /// Subtracts one byte from an other and subtracts the carry
    pub fn sub_with_carry(mut self, right: Self, mut carry: Bit) -> (Self, Bit) {
        let mut carry_byte = [Bit::Low; 8];
        carry_byte[0] = carry;
        (self, carry) = self - Self::from(carry_byte);
        let (result, carry2) = self - right;
        (result, carry.or(carry2))
    }
}

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Self {
            bits: array::from_fn(|i| Bit::from((value >> i) & 1 == 1)),
        }
    }
}

impl From<Byte> for u8 {
    fn from(value: Byte) -> Self {
        value
            .bits
            .into_iter()
            .enumerate()
            .fold(0, |result, (i, bit)| {
                result | (Self::from(bool::from(bit)) << i)
            })
    }
}

impl From<[Bit; 8]> for Byte {
    fn from(bits: [Bit; 8]) -> Self {
        Self { bits }
    }
}

impl From<Byte> for [Bit; 8] {
    fn from(value: Byte) -> Self {
        value.bits
    }
}

impl Add for Byte {
    type Output = (Self, Bit);

    /// Adds 2 bytes without carry bit
    fn add(self, rhs: Self) -> Self::Output {
        self.add_with_carry(rhs, Bit::Low)
    }
}

impl Sub for Byte {
    type Output = (Self, Bit);

    /// Subtracts one byte from an other
    fn sub(self, rhs: Self) -> Self::Output {
        self.add_with_carry(rhs.not(), Bit::High)
    }
}

impl BitAnd for Byte {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(&rhs)
    }
}

impl BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.and(&rhs);
    }
}

impl BitOr for Byte {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(&rhs)
    }
}

impl BitOrAssign for Byte {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.or(&rhs);
    }
}

impl BitXor for Byte {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(&rhs)
    }
}

impl BitXorAssign for Byte {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.xor(&rhs);
    }
}

impl Not for Byte {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            bits: array::from_fn(|index| self.bits[index].not()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bit::Bit, byte::Byte};

    #[test]
    fn byte_conversion() {
        for byte in 0..=u8::MAX {
            assert_eq!(u8::from(Byte::from(byte)), byte);
        }
    }

    #[test]
    fn add_without_carry() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from((Byte::from(left) + Byte::from(right)).0),
                    left.wrapping_add(right)
                );
            }
        }
    }

    #[test]
    fn add_with_carry_test() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from(
                        Byte::from(left)
                            .add_with_carry(Byte::from(right), Bit::High)
                            .0
                    ),
                    left.wrapping_add(right).wrapping_add(1)
                );
            }
        }
    }

    #[test]
    fn subtract() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from((Byte::from(left) - Byte::from(right)).0),
                    left.wrapping_sub(right)
                );
            }
        }
    }

    #[test]
    fn subtract_with_carry() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from(
                        Byte::from(left)
                            .sub_with_carry(Byte::from(right), Bit::High)
                            .0
                    ),
                    left.wrapping_sub(right).wrapping_sub(1)
                );
            }
        }
    }
}
