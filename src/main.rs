mod finch;
mod lexome;
mod tests;

use crate::finch::{Finch, ReturnPacket};
use crate::lexome::{Lexome, dummy_memory};

fn main() {
    let mut finch: Finch = Finch::new(0,0,0);
    finch.memory = dummy_memory();
    println!("{:?}", finch.memory);
    for i in 0..100 {
        println!("{:?}",finch.memory[finch.inst_h]);
        let return_packet: ReturnPacket = finch.increment();
        println!("{:?}", finch);
    }
}