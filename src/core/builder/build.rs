//src/core/builder/build.rs

use crate::core::io::read::Reader;
use crate::core::subleq::Computer;

pub fn build(reader: &Reader) -> Computer {
    let memory_size = reader.memory().unwrap_or(256);

    let mut computer = Computer::initialize(memory_size);

    // Load data entries
    for entry in reader.data() {
        computer.load(entry.address as usize, entry.value);
    }

    // Load instructions
    let mut address = 0;

    for instruction in reader.instructions() {
        computer.load(address, instruction.a);
        computer.load(address + 1, instruction.b);
        computer.load(address + 2, instruction.c);

        address += 3;
    }

    computer
}