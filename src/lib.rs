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
use byte::Byte;
use mux::byte::Registers;

pub mod bit;
pub mod byte;
pub mod mux;

/// The ALU executes all CPU instructions.
///
/// # Panics
/// The program panics if an invalid instruction was found or the program ended unexpectedly
pub fn alu(mut iter: impl Iterator<Item = u8>) {
    /*
    0000 00RT: Load constant
    0000 01RT: Load memory
    0000 10RF: store memory
    0001 RTRF: Move between registers
    0010 RTRF: Nand
    0011 RTRF: And
    0100 RTRF: Nor
    0101 RTRF: Or
    0110 RTRF: Xnor
    0111 RTRF: Xor
    1000 RTRF: Add
    1001 RTRF: Add with overflow
    1010 RTRF: Sub
    1011 RTRF: Sub with overflow*/
    let mut registers = Registers::new();
    let mut memory = [Byte::from(0); 256];
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
                    memory[usize::from(iter.next().expect("Unexpected end of program"))],
                );
            }
            8..12 => {
                memory[usize::from(iter.next().expect("Unexpected end of program"))] =
                    registers.load(reg_low);
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
