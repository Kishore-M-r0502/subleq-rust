//src/core/io/group.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataEntry {
    pub value: i32,
    pub address: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}