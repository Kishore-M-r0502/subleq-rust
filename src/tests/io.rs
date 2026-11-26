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
//src/tests/io.rs
use crate::core::io::{read::{Section,Reader,Terminator},errors::ReadError};
#[test]
fn test_readline_validation() {
    let mut reader = Reader::init();

    // 1. Valid section transitions
    assert_eq!(reader.readline("data:"), Ok(()));
    assert_eq!(reader.readline("end"), Ok(()));
    assert_eq!(reader.readline("code:"), Ok(()));

    // 2. Valid data declaration
    reader.updatesec(Section::Data);
    assert_eq!(reader.readline("0 10 ;"), Ok(()));

    // 3. Valid SUBLEQ instruction
    reader.updatesec(Section::Code);
    assert_eq!(reader.readline("1 11 20;;"), Ok(()));

    // 4. Invalid terminator
    assert!(matches!(
        reader.readline("1 11 20;"),
        Err(ReadError::InvalidTerminator(Terminator::Data))
    ));

    // 5. Content outside a section
    reader.updatesec(Section::None);
    assert!(matches!(
        reader.readline("1 11 20;;"),
        Err(ReadError::InvalidContext(Section::None))
    ));

    // 6. Missing terminator
    reader.updatesec(Section::Code);
    assert!(matches!(
        reader.readline("1 11 20"),
        Err(ReadError::MissingTerminator)
    ));
}

#[test]
    fn section_transitions() {
        let mut reader = Reader::init();

        reader.readsec("data:");
        assert!(matches!(reader.section, Section::Data));

        reader.readsec("end");
        assert!(matches!(reader.section, Section::None));

        reader.readsec("code:");
        assert!(matches!(reader.section, Section::Code));

        reader.readsec("end");
        assert!(matches!(reader.section, Section::None));
    }
#[test]
fn representtest() {
    let mut reader = Reader::init();

reader.readline("data:").unwrap();
reader.readline("0 10 ;").unwrap();
reader.readline("11 20 ;").unwrap();
reader.readline("end").unwrap();

reader.readline("code:").unwrap();
reader.readline("1 11 20;;").unwrap();
reader.readline("2 4 -1;;").unwrap();
reader.readline("end").unwrap();
}