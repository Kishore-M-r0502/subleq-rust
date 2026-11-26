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