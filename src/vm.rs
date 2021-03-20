#[derive(Debug, Default)]
pub(crate) struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    program_counter: usize,
}

impl VM {
    #[allow(dead_code)]
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            program_counter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VM;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();

        assert_eq!(test_vm.registers[0], 0);
        assert_eq!(test_vm.program, []);
        assert_eq!(test_vm.program_counter, 0);
    }
}
