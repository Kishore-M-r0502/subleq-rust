//src/core/io/read.rs
use crate::core::io::{errors::ReadError::{self, MissingTerminator}, group::{DataEntry, Instruction}};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    None,
    Data,
    Code,
}
#[derive(Debug)]
pub struct Reader{
    row:usize,
    col:usize,
    pub section:Section,
    data:Vec<DataEntry>,
    instructions:Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terminator {
    Data,
    Instruction,
}

impl Reader{
    pub fn init()->Self{
        Self { row: 1, col: 1 , section: Section::None ,data:Vec::new(),instructions:Vec::new()}
    }
    pub fn updatesec(&mut self,sec:Section){
        self.section=sec;
    }
    pub fn readsec(&mut self,line:&str)->bool{
        match line.trim(){
            "data:"=>{
                self.updatesec(Section::Data);
                true
            },
            "code:"=>{
                self.updatesec(Section::Code);
                true
            },
            "end"=>{
                self.updatesec(Section::None);
                true
            },
            _=>false

        }
    }

    pub fn readline(&mut self, line: &str) -> Result<(), ReadError> {
    let line = line.trim();

    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }

    if self.readsec(line) {
        return Ok(());
    }

    match self.section {
        Section::None => Err(ReadError::InvalidContext(self.section)),

        Section::Data => {
            let entry = self.dataparse(line)?;
            self.data.push(entry);
            Ok(())
        }

        Section::Code => {
            let instruction = self.iparse(line)?;
            self.instructions.push(instruction);
            Ok(())
        }
    }
}
fn read_terminator(&self, line: &str) -> Result<Terminator,ReadError>{
    match line.trim(){
        s if s.ends_with(";;")=>Ok(Terminator::Instruction),
        s if s.ends_with(";")=>Ok(Terminator::Data),
        _=> Err(ReadError::MissingTerminator)
    }
}

pub fn checkterm(&self,term:Terminator)->Result<(),ReadError>{
    match(self.section,term){
        (Section::Code,Terminator::Instruction)=>Ok(()),
        (Section::Data,Terminator::Data)=>Ok(()),
        (Section::None,_)=>Err(ReadError::InvalidContext(self.section)),
        (Section::Data,Terminator::Instruction)=>Err(ReadError::InvalidTerminator(term)),
        (Section::Code,Terminator::Data)=>Err(ReadError::InvalidTerminator(term))
    }
}

pub fn dataparse(&mut self, line: &str) -> Result<DataEntry, ReadError> {
    match line.strip_suffix(';') {
        Some(data) => {
            let tokens: Vec<&str> = data.split_whitespace().collect();

            if tokens.len() != 2 {
                return Err(ReadError::InvalidDataFormat);
            }

            let address = match tokens[0].parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ReadError::InvalidNumber),
            };

            let value = match tokens[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ReadError::InvalidNumber),
            };

            Ok(DataEntry { address, value })
        }
        None => {
    if line.ends_with(";;") {
        Err(ReadError::InvalidTerminator(Terminator::Data))
    } else {
        Err(ReadError::MissingTerminator)
    }
}
    }
}
pub fn iparse(&mut self, line: &str) -> Result<Instruction, ReadError> {
    match line.strip_suffix(";;") {
        Some(data) => {
            let tokens: Vec<&str> = data.split_whitespace().collect();

            if tokens.len() != 3 {
                return Err(ReadError::InvalidInstructionFormat);
            }

            let a = match tokens[0].parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ReadError::InvalidNumber),
            };

            let b = match tokens[1].parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ReadError::InvalidNumber),
            };

            let c = match tokens[2].parse::<i32>() {
                Ok(n) => n,
                Err(_) => return Err(ReadError::InvalidNumber),
            };

            Ok(Instruction { a, b, c })
        }
        None => {
            if line.ends_with(';') {
        Err(ReadError::InvalidTerminator(Terminator::Data))
    } else {
        Err(ReadError::MissingTerminator)
    }
    }
}
}
}