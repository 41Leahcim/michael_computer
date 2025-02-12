//! Circuits interacting on bytes

use core::array;

use crate::{bit::Bit, byte::Byte};

use super::bit::full_adder;

/// Adds 2 bytes and the carry bit
pub fn byte_add_with_carry(left: Byte, right: Byte, mut carry: Bit) -> (Byte, Bit) {
    let left: [Bit; 8] = left.into();
    let right: [Bit; 8] = right.into();
    (
        Byte::from(array::from_fn(|i| {
            let result;
            (result, carry) = full_adder(left[i], right[i], carry);
            result
        })),
        carry,
    )
}

/// Adds 2 bytes without carry bit
pub fn byte_add(left: Byte, right: Byte) -> (Byte, Bit) {
    byte_add_with_carry(left, right, Bit::Low)
}

#[cfg(test)]
mod test {
    use crate::{
        bit::Bit,
        byte::Byte,
        circuit::byte::{byte_add, byte_add_with_carry},
    };

    #[test]
    fn add_without_carry() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from(byte_add(Byte::from(left), Byte::from(right)).0),
                    left.wrapping_add(right)
                );
            }
        }
    }

    #[test]
    fn add_with_carry() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from(byte_add_with_carry(Byte::from(left), Byte::from(right), Bit::High).0),
                    left.wrapping_add(right).wrapping_add(1)
                );
            }
        }
    }
}
