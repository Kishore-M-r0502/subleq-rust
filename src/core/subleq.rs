// subleq.rs at src/core/subleq.rs
use crate::core::tracer::trace::{State, Tracer};
pub const MEMORY_MAX:usize=256;
#[derive(Debug)]
pub struct Computer{
    memory:[i32;MEMORY_MAX],
    pc:isize,
}

impl Computer{
    pub fn initialize()->Self{
        Self { memory: [0;MEMORY_MAX] ,pc:0}
    }
    pub fn run(&mut self){
        self.header();
        let mut ic=0;
        
        while self.pc >= 0 && self.pc + 2 < MEMORY_MAX as isize{
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
    }

pub fn load(&mut self, index: usize, value: i32) {
    self.memory[index] = value;
}
pub fn fetch(&self, index: usize) -> i32 {
    self.memory[index]
}
}