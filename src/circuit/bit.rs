//! Circuits interacting on bits

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

/// Returns input bit as left bit, if select is `Bit::Low`, returns input bit as right bit
/// otherwise. Other bit will be `Bit::Low`.
pub const fn dmux(input: Bit, select: Bit) -> (Bit, Bit) {
    (input.and(select.not()), input.and(select))
}

#[cfg(test)]
mod tests {
    use super::{full_adder, half_adder};
    use crate::{
        bit::Bit,
        circuit::bit::{dmux, mux},
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
    fn dmux_test() {
        assert_eq!(dmux(Bit::Low, Bit::Low), (Bit::Low, Bit::Low));
        assert_eq!(dmux(Bit::Low, Bit::High), (Bit::Low, Bit::Low));
        assert_eq!(dmux(Bit::High, Bit::Low), (Bit::High, Bit::Low));
        assert_eq!(dmux(Bit::High, Bit::High), (Bit::Low, Bit::High));
    }
}
