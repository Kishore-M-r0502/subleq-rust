// This file is part of SUBLEQ Rust.
//
// Copyright (C) 2026 Kishore M R
//
// SUBLEQ Rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// SUBLEQ Rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with SUBLEQ Rust. If not, see <https://www.gnu.org/licenses/>.
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