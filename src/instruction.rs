//! Bytecode instructions for `CornCob` VM.

use crate::opcode::Opcode;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// `Instruction` is a structure containing the selected `Opcode` enum variant, and arguments to
/// that opcode.
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    #[must_use]
    /// `Instruction::new` initializes the Instruction struct.
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, opcode::Opcode};

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
