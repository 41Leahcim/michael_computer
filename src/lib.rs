#![no_std]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_const_for_fn,
    missing_docs
)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

//! This library contains the implementation of gates, circuits, and datatypes used by the computer

use bit::Bit;
use byte::Byte;

pub mod bit;
pub mod byte;
pub mod mux;

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
    let mut registers = [Byte::from(0); 4];
    let mut memory = [Byte::from(0); 256];
    let mut overflow = Bit::Low;
    while let Some(byte) = iter.next() {
        let reg_low = usize::from(byte % 4);
        let reg_high = usize::from(byte / 4 % 4);
        match byte {
            0..4 => registers[reg_low] = iter.next().expect("Unexpected end of program").into(),
            4..8 => {
                registers[reg_low] =
                    memory[usize::from(iter.next().expect("Unexpected end of program"))];
            }
            8..12 => {
                memory[usize::from(iter.next().expect("Unexpected end of program"))] =
                    registers[reg_low];
            }
            12..16 => registers[reg_low] = !registers[reg_low],
            16..32 => registers[reg_high] = registers[reg_low],
            32..48 => registers[reg_high] = registers[reg_high].nand(&registers[reg_low]),
            48..64 => registers[reg_high] &= registers[reg_low],
            64..80 => registers[reg_high] = registers[reg_high].nor(&registers[reg_low]),
            80..96 => registers[reg_high] |= registers[reg_low],
            96..112 => registers[reg_high] = registers[reg_high].xnor(&registers[reg_low]),
            112..128 => registers[reg_high] ^= registers[reg_low],
            128..144 => {
                let (result, carry) = registers[reg_high] + registers[reg_low];
                registers[reg_high] = result;
                overflow = carry;
            }
            144..160 => {
                let (result, carry) =
                    registers[reg_high].add_with_carry(registers[reg_low], overflow);
                registers[reg_high] = result;
                overflow = carry;
            }
            160..176 => {
                let (result, carry) = registers[reg_high] - registers[reg_low];
                registers[reg_high] = result;
                overflow = carry;
            }
            176..192 => {
                let (result, carry) =
                    registers[reg_high].sub_with_carry(registers[reg_low], overflow);
                registers[reg_high] = result;
                overflow = carry;
            }
            192.. => panic!("Invalid instruction: {byte}"),
        }
    }
}
