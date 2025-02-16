//! Circuits interacting on bytes

use core::array;

use crate::{bit::Bit, byte::Byte};

use super::bit::{full_adder, mux as bit_mux};

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

/// Subtracts one byte from an other
pub fn byte_sub(left: Byte, right: Byte) -> (Byte, Bit) {
    byte_add_with_carry(left, right.not(), Bit::High)
}

/// Subtracts one byte from an other and subtracts the carry
pub fn byte_sub_with_carry(mut left: Byte, right: Byte, mut carry: Bit) -> (Byte, Bit) {
    let mut carry_byte = [Bit::Low; 8];
    carry_byte[0] = carry;
    (left, carry) = byte_sub(left, Byte::from(carry_byte));
    let (result, carry2) = byte_sub(left, right);
    (result, carry.or(carry2))
}

/// Returns the left byte if `select` is `Bit::Low`, returns right byte otherwise
pub fn mux(left: Byte, right: Byte, select: Bit) -> Byte {
    let left: [Bit; 8] = left.into();
    let right: [Bit; 8] = right.into();
    Byte::from(array::from_fn(|i| bit_mux(left[i], right[i], select)))
}

/// `select[0]` adds 1 to the index if `Bit::High`.
/// `select[1]` adds 2 to the index if `Bit::High`.
/// Returns the byte at the resulting index.
pub fn mux4(input: [Byte; 4], select: [Bit; 2]) -> Byte {
    mux(
        mux(input[0], input[1], select[0]),
        mux(input[2], input[3], select[0]),
        select[1],
    )
}

/// Every select bit adds (1 << index) if `Bit::High`.
/// Returns the byte at the resulting index
#[expect(clippy::missing_panics_doc)]
pub fn mux16(input: [Byte; 16], select: [Bit; 4]) -> Byte {
    mux4(
        array::from_fn(|i| {
            mux4(
                input[i * 4..i * 4 + 4].try_into().unwrap(),
                select[..2].try_into().unwrap(),
            )
        }),
        select[2..4].try_into().unwrap(),
    )
}

/// Every select bit adds (1 << index) if `Bit::High`.
/// Returns the byte at the resulting index
#[expect(clippy::missing_panics_doc)]
pub fn mux256(input: [Byte; 256], select: [Bit; 8]) -> Byte {
    mux16(
        array::from_fn(|i| {
            mux16(
                input[i * 16..i * 16 + 16].try_into().unwrap(),
                select[..4].try_into().unwrap(),
            )
        }),
        select[4..8].try_into().unwrap(),
    )
}

/// Returns input bit as left bit, if select is `Bit::Low`, returns input bit as right bit
/// otherwise. Other bit will be `Bit::Low`.
pub fn dmux(input: Byte, select: Bit) -> (Byte, Byte) {
    let input: [Bit; 8] = input.into();
    (
        Byte::from(array::from_fn(|i| input[i].and(select.not()))),
        Byte::from(array::from_fn(|i| input[i].and(select))),
    )
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux4(input: Byte, select: [Bit; 2]) -> [Byte; 4] {
    let input: [Bit; 8] = input.into();
    array::from_fn(|i| {
        Byte::from(array::from_fn(|j| {
            Bit::from(
                select
                    .iter()
                    .enumerate()
                    .all(|(k, bit)| &Bit::from((i >> k) & 1 == 1) == bit),
            )
            .and(input[j])
        }))
    })
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux16(input: Byte, select: [Bit; 4]) -> [Byte; 16] {
    let input: [Bit; 8] = input.into();
    array::from_fn(|i| {
        Byte::from(array::from_fn(|j| {
            Bit::from(
                select
                    .iter()
                    .enumerate()
                    .all(|(k, bit)| &Bit::from((i >> k) & 1 == 1) == bit),
            )
            .and(input[j])
        }))
    })
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux256(input: Byte, select: [Bit; 8]) -> [Byte; 256] {
    let input: [Bit; 8] = input.into();
    array::from_fn(|i| {
        Byte::from(array::from_fn(|j| {
            Bit::from(
                select
                    .iter()
                    .enumerate()
                    .all(|(k, bit)| &Bit::from((i >> k) & 1 == 1) == bit),
            )
            .and(input[j])
        }))
    })
}

#[cfg(test)]
mod test {
    use crate::{
        bit::Bit,
        byte::Byte,
        circuit::byte::{byte_add, byte_add_with_carry, byte_sub, byte_sub_with_carry},
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

    #[test]
    fn subtract() {
        for left in 0..=u8::MAX {
            for right in 0..=u8::MAX {
                assert_eq!(
                    u8::from(byte_sub(Byte::from(left), Byte::from(right)).0),
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
                    u8::from(byte_sub_with_carry(Byte::from(left), Byte::from(right), Bit::High).0),
                    left.wrapping_sub(right).wrapping_sub(1)
                );
            }
        }
    }
}
