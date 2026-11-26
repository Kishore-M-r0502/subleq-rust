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
// subleq.rs at src/core/subleq.rs
use crate::core::tracer::trace::{State, Tracer};
pub const START_VALUE:i32=0;
#[derive(Debug)]
pub struct Computer{
    memory:Vec<i32>,
    pc:isize,
}

impl Computer{
    pub fn initialize(memory:usize)->Self{
        Self { memory: vec![START_VALUE;memory],pc:0}
    }
    pub fn checkmem(&self)->usize{
        return self.memory.len()
    }
    

pub fn load(&mut self, index: usize, value: i32) {
    self.memory[index] = value;
}
pub fn fetch(&self, index: usize) -> i32 {
    self.memory[index]
}
pub fn clear(&mut self,loc:usize){
    self.memory[loc]=START_VALUE;
}
pub fn next(&mut self)->isize{
    let mut ic=0;
    self.header();
    while self.pc>=0 && self.pc+2 <self.memory.len() as isize{
    let a=self.memory[self.pc as usize] as usize;
        let b=self.memory[(self.pc+1) as usize] as usize;
        let c=self.memory[(self.pc+2) as usize] as isize;
        self.memory[b]=self.memory[b]-self.memory[a];
        if c==-1{
                self.trace(ic, self.pc, a, b, c, self.memory[b], State::HALT);
                break;
            }
            else if self.memory[b]<=0 {
                self.trace(ic, self.pc, a, b, c, self.memory[b], State::BRANCH);
                ic+=1;
                self.pc=c;
            }
            else {
                self.trace(ic, self.pc, a, b, c, self.memory[b], State::NEXT);
                ic+=1;
                self.pc+=3;
            }
        }
    return self.pc;
        
}
}