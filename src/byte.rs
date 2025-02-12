//! Contains the byte datatype with gates.

use core::array;

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

    /// Inverts the byte
    pub fn not(&self) -> Self {
        Self {
            bits: array::from_fn(|index| self.bits[index].not()),
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

#[cfg(test)]
mod tests {
    use crate::byte::Byte;

    #[test]
    fn byte_conversion() {
        for byte in 0..=u8::MAX {
            assert_eq!(u8::from(Byte::from(byte)), byte);
        }
    }
}
