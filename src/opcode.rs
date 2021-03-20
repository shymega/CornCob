#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Opcode {
    /// Halt virtual machine
    HLT,
    /// Illegal instruction
    IGL,
    /// No Operation - do nothing
    NOP,
}
