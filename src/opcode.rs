//! Module of VM opcodes.

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(dead_code, clippy::upper_case_acronyms, non_camel_case_types)]
/// `Opcode` is an enum defining different opcodes, with their bytecode identifier.
pub enum Opcode {
    /// Halt virtual machine.
    HLT = 0x0,
    /// Illegal instruction.
    IGL = 0x1,
    /// No Operation - do nothing.
    NOP = 0x2,
}

impl Default for Opcode {
    fn default() -> Self {
        Self::NOP
    }
}
