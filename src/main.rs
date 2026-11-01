#[cfg(test)]
mod tests;
use crate::core::subleq::Computer;
use crate::core::io::read::Reader;
mod core;
fn main(){
    let mut computer = Computer::initialize();

    // computes 5+3=8
    computer.load(0, 10);
    computer.load(1, 12);
    computer.load(2, 3);

    computer.load(3, 12);
    computer.load(4, 11);
    computer.load(5, -1);

    computer.load(10, -7);
    computer.load(11, 12);
    computer.load(12, 0);

    computer.next();
    




}