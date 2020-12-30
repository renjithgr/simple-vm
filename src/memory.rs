pub struct Memory {
    memory: Vec<u8>
}

impl Memory {

    fn new(capcity: usize) -> Self {
        Memory {
            memory: vec!(0; capcity)
        }
    }

    fn set_memory(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    fn get_memory_u8(&self, index: usize) -> u8 {
        return self.memory[index];
    }

    fn get_memory_u16(&self, index: usize) -> u16 {
        // https://stackoverflow.com/questions/50243866/how-do-i-convert-two-u8-primitives-into-a-u16-primitive
        return u16::from_be_bytes([self.memory[index], self.memory[index + 1]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_memory_value() {
        let memory = Memory::new(10);

        let acc = memory.get_memory_u8(1);

        assert_eq!(acc, 0);
    }

    #[test]
    fn should_read_memory_value_u16() {
        let mut memory = Memory::new(10);

        memory.set_memory(0, 0x12);
        memory.set_memory(1, 0x34);

        assert_eq!(memory.get_memory_u16(0), 0x1234);
    }

    #[test]
    fn should_set_memory_value() {
        let mut memory = Memory::new(10);

        memory.set_memory(0, 0x12);
        memory.set_memory(1, 0x34);

        assert_eq!(memory.get_memory_u8(0), 0x12);
        assert_eq!(memory.get_memory_u8(1), 0x34);
    }
}