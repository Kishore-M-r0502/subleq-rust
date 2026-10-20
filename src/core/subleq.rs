// subleq.rs at src/core/subleq.rs
pub const MEMORY_MAX:usize=256;
#[derive(Debug)]
pub struct Memory{
    memory:[i32;MEMORY_MAX],
    pc:isize,
}


impl Memory{
    pub fn new()->Self{
        Self { memory: [0;256] ,pc:0}
    }
    pub fn start(&mut self){
        
        while self.pc >= 0 && self.pc + 2 < MEMORY_MAX as isize{
        let a=self.memory[self.pc as usize] as usize;
        let b=self.memory[(self.pc+1) as usize] as usize;
        let c=self.memory[(self.pc+2) as usize] as isize;
            self.memory[b]=self.memory[b]-self.memory[a];
            if c==-1{
                break;
            }
            else if self.memory[b]<=0 {
                self.pc=c;
            }
            else {
                self.pc+=3;
            }
        }
    }

pub fn set(&mut self, index: usize, value: i32) {
    self.memory[index] = value;
}
pub fn get(&self, index: usize) -> i32 {
    self.memory[index]
}
}