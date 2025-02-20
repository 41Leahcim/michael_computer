//! This library contains the instructions and registers for the assembly language.

#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::missing_const_for_fn,
    missing_docs
)]

/// The type representing a register index
#[expect(missing_docs)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        match value {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
        }
    }
}

/// The instructions for the computer
pub enum Instruction {
    /// Loads the next byte in the program as value for the register.
    LoadConstant(Register, u8),

    /// Loads the byte from the address represented by the next byte from memory into the register.
    LoadMemory(Register, u8),

    /// Stores the value of the register at the address represented by the next byte.
    StoreMemory(Register, u8),

    /// Inverts the value of the register.
    Not(Register),

    /// Assingns the left register the value of the right register.
    Move(Register, Register),

    /// Performs the nand operation between the 2 registers, stores the result in the left register.
    Nand(Register, Register),

    /// Performs the and operation between the 2 registers, stores the result in the left register.
    And(Register, Register),

    /// Performs the nor operation between the 2 registers, stores the result in the left register.
    Nor(Register, Register),

    /// Performs the or operation between the 2 registers, stores the result in the left register.
    Or(Register, Register),

    /// Performs the xnor operation between the 2 registers, stores the result in the left register.
    Xnor(Register, Register),

    /// Performs the xor operation between the 2 registers, stores the result in the left register.
    Xor(Register, Register),

    /// Adds the 2 registers, stores the result in the left register, stores whether it overflowed.
    Add(Register, Register),

    /// Adds the 2 register + overflow, stores the result in the left register, stores whether it
    /// overflowed.
    AddOverflow(Register, Register),

    /// Subtracts the right register to the left, stores the result in the left register, stores
    /// whether it overflowed.
    Sub(Register, Register),

    /// Subtracts the right register to the left and overflow, stores the result in the left
    /// register, stores whether it overflowed.
    SubOverflow(Register, Register),
}

/*
0000 00RT | Load constant
0000 01RT | Load memory
0000 10RF | store memory
0000 11RR | Not
0001 RTRF | Move between registers
0010 RTRF | Nand
0011 RTRF | And
0100 RTRF | Nor
0101 RTRF | Or
0110 RTRF | Xnor
0111 RTRF | Xor
1000 RTRF | Add
1001 RTRF | Add with overflow
1010 RTRF | Sub
1011 RTRF | Sub with overflow
*/
impl Instruction {
    /// Converts a stream of instructions to a vector of bytes
    pub fn to_bytes(instructions: impl IntoIterator<Item = Self>) -> Vec<u8> {
        instructions
            .into_iter()
            .fold(Vec::new(), |mut result, instruction| {
                match instruction {
                    Self::LoadConstant(register, value) => {
                        result.extend_from_slice(&[u8::from(register), value]);
                    }
                    Self::LoadMemory(register, address) => {
                        result.extend_from_slice(&[4 | u8::from(register), address]);
                    }
                    Self::StoreMemory(register, address) => {
                        result.extend_from_slice(&[8 | u8::from(register), address]);
                    }
                    Self::Not(register) => result.extend_from_slice(&[12 | u8::from(register)]),
                    Self::Move(register, register1) => result
                        .extend_from_slice(&[16 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Nand(register, register1) => result
                        .extend_from_slice(&[32 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::And(register, register1) => result
                        .extend_from_slice(&[48 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Nor(register, register1) => result
                        .extend_from_slice(&[64 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Or(register, register1) => result
                        .extend_from_slice(&[80 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Xnor(register, register1) => result
                        .extend_from_slice(&[96 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Xor(register, register1) => result
                        .extend_from_slice(&[112 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Add(register, register1) => result
                        .extend_from_slice(&[128 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::AddOverflow(register, register1) => result
                        .extend_from_slice(&[144 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::Sub(register, register1) => result
                        .extend_from_slice(&[160 | (u8::from(register) << 2), u8::from(register1)]),
                    Self::SubOverflow(register, register1) => result
                        .extend_from_slice(&[172 | (u8::from(register) << 2), u8::from(register1)]),
                }
                result
            })
    }
}
