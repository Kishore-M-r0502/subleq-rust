use crate::core::subleq::Memory;
mod core;
fn main(){
    let mut machine = Memory::new();

    // computes 5+3=8
    machine.set(0, 10);
    machine.set(1, 12);
    machine.set(2, 3);

    machine.set(3, 12);
    machine.set(4, 11);
    machine.set(5, -1);

    machine.set(10, 5);
    machine.set(11, 3);
    machine.set(12, 0);

    machine.start();
    println!("{:?}",machine.get(11));


}