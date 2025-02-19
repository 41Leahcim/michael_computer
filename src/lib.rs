#![no_std]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_const_for_fn,
    missing_docs
)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

//! This library contains the implementation of gates, circuits, and datatypes used by the computer

use core::array;

use bit::Bit;
use mux::byte::{Ram, Registers};

pub mod bit;
pub mod byte;
pub mod mux;

/// The ALU executes all CPU instructions.
///
/// # Panics
/// The program panics if an invalid instruction was found or the program ended unexpectedly
pub fn alu(mut iter: impl Iterator<Item = u8>) {
    let mut registers = Registers::new();
    let mut memory = Ram::new();

    let mut overflow = Bit::Low;
    while let Some(byte) = iter.next() {
        let reg_low = array::from_fn(|i| Bit::from((byte >> i) & 1 == 1));
        let reg_high = array::from_fn(|i| Bit::from((byte >> (i + 2)) & 1 == 1));
        match byte {
            0..4 => registers.store(
                reg_low,
                iter.next().expect("Unexpected end of program").into(),
            ),
            4..8 => {
                registers.store(
                    reg_low,
                    memory.load(iter.next().expect("Unexpected end of program").into()),
                );
            }
            8..12 => {
                memory.store(
                    iter.next().expect("Unexpected end of program").into(),
                    registers.load(reg_low),
                );
            }
            12..16 => registers.store(reg_low, !registers.load(reg_low)),
            16..32 => registers.store(reg_high, registers.load(reg_low)),
            32..48 => registers.store(
                reg_high,
                registers.load(reg_high).nand(&registers.load(reg_low)),
            ),
            48..64 => registers.store(reg_high, registers.load(reg_high) & registers.load(reg_low)),
            64..80 => registers.store(
                reg_high,
                registers.load(reg_high).nor(&registers.load(reg_low)),
            ),
            80..96 => registers.store(reg_high, registers.load(reg_high) | registers.load(reg_low)),
            96..112 => registers.store(
                reg_high,
                registers.load(reg_high).xnor(&registers.load(reg_low)),
            ),
            112..128 => {
                registers.store(reg_high, registers.load(reg_high) ^ registers.load(reg_low));
            }
            128..144 => {
                let (result, carry) = registers.load(reg_high) + registers.load(reg_low);
                registers.store(reg_high, result);
                overflow = carry;
            }
            144..160 => {
                let (result, carry) = registers
                    .load(reg_high)
                    .add_with_carry(registers.load(reg_low), overflow);
                registers.store(reg_high, result);
                overflow = carry;
            }
            160..176 => {
                let (result, carry) = registers.load(reg_high) - registers.load(reg_low);
                registers.store(reg_high, result);
                overflow = carry;
            }
            176..192 => {
                let (result, carry) = registers
                    .load(reg_high)
                    .sub_with_carry(registers.load(reg_low), overflow);
                registers.store(reg_high, result);
                overflow = carry;
            }
            192.. => panic!("Invalid instruction: {byte}"),
        }
    }
}
