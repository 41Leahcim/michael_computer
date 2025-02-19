//! Circuits interacting on bytes

use core::array;

use crate::{bit::Bit, byte::Byte};

use super::bit::{self, mux as bit_mux};

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

/// Simple 256 byte RAM memory
pub struct Ram {
    data: [Byte; 256],
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}

impl Ram {
    /// Initializes memory
    pub fn new() -> Self {
        Self {
            data: [Byte::from(0); 256],
        }
    }

    /// Loads a byte from memory
    pub fn load(&self, address: Byte) -> Byte {
        mux256(self.data, address.into())
    }

    /// Stores the new byte in memory
    pub fn store(&mut self, address: Byte, value: Byte) {
        let new_value = dmux256(value, address.into());
        let select = bit::dmux256(Bit::High, address.into());
        for ((target, value), select) in self.data.iter_mut().zip(new_value).zip(select) {
            *target = mux(*target, value, select);
        }
    }
}

/// A simple set of registers
pub struct Registers {
    data: [Byte; 4],
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    /// Initializes the registers
    pub fn new() -> Self {
        Self {
            data: [Byte::from(0); 4],
        }
    }

    /// Loads the value of a register
    pub fn load(&self, select: [Bit; 2]) -> Byte {
        mux4(self.data, select)
    }

    /// Stores the new byte in a register
    pub fn store(&mut self, select: [Bit; 2], value: Byte) {
        let new_value = dmux4(value, select);
        let select = bit::dmux4(Bit::High, select);
        for ((target, value), select) in self.data.iter_mut().zip(new_value).zip(select) {
            *target = mux(*target, value, select);
        }
    }
}
