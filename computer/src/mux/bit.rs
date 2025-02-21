//! Muxes interacting on bits

use core::array;

use crate::bit::Bit;

/// Adds 2 bits and returns a sum and a carry bit
pub const fn half_adder(left: Bit, right: Bit) -> (Bit, Bit) {
    (left.xor(right), left.and(right))
}

/// Adds 3 bits (one of which a carry bit), and returns a sum and a new carry bit
pub const fn full_adder(left: Bit, right: Bit, carry: Bit) -> (Bit, Bit) {
    (
        left.xor(right).xor(carry),
        left.xor(right).and(carry).or(left.and(right)),
    )
}

/// Returns the left bit if `select` is `Bit::Low`, returns right bit otherwise
pub const fn mux(left: Bit, right: Bit, select: Bit) -> Bit {
    left.and(select.not()).or(right.and(select))
}

/// `select[0]` adds 1 to the index if `Bit::High`.
/// `select[1]` adds 2 to the index if `Bit::High`.
/// Returns the bit at the resulting index.
pub const fn mux4(input: [Bit; 4], select: [Bit; 2]) -> Bit {
    mux(
        mux(input[0], input[1], select[0]),
        mux(input[2], input[3], select[0]),
        select[1],
    )
}

/// Every select bit adds (1 << index) if `Bit::High`.
/// Returns the bit at the resulting index
#[expect(clippy::missing_panics_doc)]
pub fn mux16(input: [Bit; 16], select: [Bit; 4]) -> Bit {
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
/// Returns the bit at the resulting index
#[expect(clippy::missing_panics_doc)]
pub fn mux256(input: [Bit; 256], select: [Bit; 8]) -> Bit {
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
pub const fn dmux(input: Bit, select: Bit) -> (Bit, Bit) {
    (input.and(select.not()), input.and(select))
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux4(input: Bit, select: [Bit; 2]) -> [Bit; 4] {
    array::from_fn(|i| {
        Bit::from(
            select
                .iter()
                .enumerate()
                .all(|(j, bit)| &Bit::from((i >> j) & 1 == 1) == bit),
        )
        .and(input)
    })
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux16(input: Bit, select: [Bit; 4]) -> [Bit; 16] {
    array::from_fn(|i| {
        Bit::from(
            select
                .iter()
                .enumerate()
                .all(|(j, bit)| &Bit::from((i >> j) & 1 == 1) == bit),
        )
        .and(input)
    })
}

/// Returns input bit as selected bit.
/// Other bits will be `Bit::Low`.
/// select[0] is 1, every next index is twice as high as the previous.
pub fn dmux256(input: Bit, select: [Bit; 8]) -> [Bit; 256] {
    array::from_fn(|i| {
        Bit::from(
            select
                .iter()
                .enumerate()
                .all(|(j, bit)| &Bit::from((i >> j) & 1 == 1) == bit),
        )
        .and(input)
    })
}

#[cfg(test)]
mod tests {
    use core::array;

    use super::{dmux4, full_adder, half_adder, mux4};
    use crate::{
        bit::Bit,
        mux::bit::{dmux, mux},
    };

    #[test]
    fn half_adder_test() {
        assert_eq!(half_adder(Bit::Low, Bit::Low), (Bit::Low, Bit::Low));
        assert_eq!(half_adder(Bit::Low, Bit::High), (Bit::High, Bit::Low));
        assert_eq!(half_adder(Bit::High, Bit::Low), (Bit::High, Bit::Low));
        assert_eq!(half_adder(Bit::High, Bit::High), (Bit::Low, Bit::High));
    }

    #[test]
    fn full_adder_test() {
        assert_eq!(
            full_adder(Bit::Low, Bit::Low, Bit::Low),
            (Bit::Low, Bit::Low)
        );
        assert_eq!(
            full_adder(Bit::Low, Bit::Low, Bit::High),
            (Bit::High, Bit::Low)
        );
        assert_eq!(
            full_adder(Bit::Low, Bit::High, Bit::Low),
            (Bit::High, Bit::Low)
        );
        assert_eq!(
            full_adder(Bit::Low, Bit::High, Bit::High),
            (Bit::Low, Bit::High)
        );
        assert_eq!(
            full_adder(Bit::High, Bit::Low, Bit::Low),
            (Bit::High, Bit::Low)
        );
        assert_eq!(
            full_adder(Bit::High, Bit::Low, Bit::High),
            (Bit::Low, Bit::High)
        );
        assert_eq!(
            full_adder(Bit::High, Bit::High, Bit::Low),
            (Bit::Low, Bit::High)
        );
        assert_eq!(
            full_adder(Bit::High, Bit::High, Bit::High),
            (Bit::High, Bit::High)
        );
    }

    #[test]
    fn mux_test() {
        assert!(!bool::from(mux(Bit::Low, Bit::Low, Bit::Low)));
        assert!(!bool::from(mux(Bit::Low, Bit::Low, Bit::High)));
        assert!(!bool::from(mux(Bit::Low, Bit::High, Bit::Low)));
        assert!(bool::from(mux(Bit::Low, Bit::High, Bit::High)));
        assert!(bool::from(mux(Bit::High, Bit::Low, Bit::Low)));
        assert!(!bool::from(mux(Bit::High, Bit::Low, Bit::High)));
        assert!(bool::from(mux(Bit::High, Bit::High, Bit::Low)));
        assert!(bool::from(mux(Bit::High, Bit::High, Bit::High)));
    }

    #[test]
    fn mux4_test() {
        for i in 0..64 {
            let input = array::from_fn(|j| Bit::from((i >> j) & 1 == 1));
            let select = array::from_fn(|j| Bit::from((i >> (j + 4)) & 1 == 1));
            assert_eq!(
                mux4(input, select),
                Bit::from(((i % 16) >> (i / 16)) & 1 == 1)
            );
        }
    }

    #[test]
    fn dmux_test() {
        assert_eq!(dmux(Bit::Low, Bit::Low), (Bit::Low, Bit::Low));
        assert_eq!(dmux(Bit::Low, Bit::High), (Bit::Low, Bit::Low));
        assert_eq!(dmux(Bit::High, Bit::Low), (Bit::High, Bit::Low));
        assert_eq!(dmux(Bit::High, Bit::High), (Bit::Low, Bit::High));
    }

    #[test]
    fn dmux4_test() {
        for i in 0..8 {
            let input = Bit::from(i & 4 == 4);
            let select = array::from_fn(|j| Bit::from((i >> j) & 1 == 1));
            let output = dmux4(input, select);
            if input == Bit::Low {
                assert_eq!(output, [Bit::Low; 4]);
            } else {
                assert_eq!(output[i % 4], Bit::High);
                assert!(output[..i % 4]
                    .iter()
                    .chain(&output[i % 4 + 1..])
                    .all(|bit| bit == &Bit::Low));
            }
        }
    }
}
