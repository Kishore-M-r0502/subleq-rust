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
//src/main.rs
#[cfg(test)]
mod tests;

use crate::core::io::read::Reader;
use crate::core::builder::build::build;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod core;

fn main() {
    // Open the SUBLEQ source file.
    let args: Vec<String> = std::env::args().collect();

if args.len() < 2 {
    eprintln!("Usage: subleq-rust <filename>");
    return;
}

let filename = &args[1];

let file = File::open(filename).unwrap();
    let buffered = BufReader::new(file);

    // Parse the complete file.
    let mut reader = Reader::init();

    for line in buffered.lines() {
        let line = line.unwrap();
        reader.readline(&line).unwrap();
    }

    // Construct the computer from the parsed program.
    let mut computer = build(&reader);
    computer.next();
    
}
    // let mut computer = Computer::initialize(256);

    // // computes 5+3=8
    // computer.load(0, 10);
    // computer.load(1, 12);
    // computer.load(2, 3);

    // computer.load(3, 12);
    // computer.load(4, 11);
    // computer.load(5, -1);

    // computer.load(10, -7);
    // computer.load(11, 12);
    // computer.load(12, 0);

    // computer.next();




