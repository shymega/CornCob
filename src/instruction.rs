use super::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    #[allow(dead_code)]
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hlt() {
        let opcode = Opcode::HLT;

        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_hlt_instruction() {
        let instruction = Instruction::new(Opcode::HLT);

        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
