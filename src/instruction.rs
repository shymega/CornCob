//! Bytecode instructions for `CornCob` VM.

use super::opcode::Opcode;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    #[must_use]
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, Opcode};

    #[test]
    fn test_hlt_opcode() {
        let opcode = Opcode::HLT;

        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_hlt_instruction() {
        let instruction = Instruction::new(Opcode::HLT);

        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
