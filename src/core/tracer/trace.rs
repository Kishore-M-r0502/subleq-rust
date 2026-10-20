// trace.rs at src/core/tracer/trace.rs

use crate::core::subleq::Computer;
#[derive(Debug)]
pub enum State{
    HALT,
    BRANCH,
    NEXT,
}
impl State{
    fn convert_string(state:&State)->String{
        match state {
            State::BRANCH=>String::from("BRANCH"),
            State::HALT=>String::from("HALT"),
            State::NEXT=>String::from("NEXT"),
        }
    }
    
}
pub trait Tracer {
    fn header(&self)->();
    fn trace(&self,ic:u32,pc:isize,a:usize,b:usize,c:isize,current:i32,state:State)->();
}
impl Tracer for Computer {
    fn header(&self)->() {
        println!("{:<5} {:<5} {:<5} {:<5} {:<5} {:<5} {}","IC","PC","a","b","c","AFTER" ,"ACTION");
    }
    fn trace(&self,ic:u32,pc:isize,a:usize,b:usize,c:isize,current:i32,state:State)->() {
        println!("{:<5} {:<5} {:<5} {:<5} {:<5} {:<5} {}",ic,pc,a,b,c,current,State::convert_string(&state));
    }
    
}