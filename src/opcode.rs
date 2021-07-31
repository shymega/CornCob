//! Module of VM opcodes.

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    /// Halt virtual machine
    HLT,
    /// Illegal instruction
    IGL,
    /// No Operation - do nothing
    NOP,
}
