mod finch;
mod lexome;
mod tests;

use crate::finch::{Finch, inc_h_non_mut, ReturnPacket};
use crate::lexome::Lexome;

fn main() {
    let mut finch: Finch = Finch::new(0,0,0);
    finch.memory = finch::dummy_memory();
    println!("{:?}", finch.memory);
    for i in 0..100 {
        println!("{:?}",finch.memory[finch.inst_h]);
        let return_packet: ReturnPacket = finch.increment();
        println!("{:?}", finch);
    }
    rules_func(finch);
}


fn rules_func(finch: Finch) {
    // mem length cannot exceed max
    if finch.memory.len() > finch.max_alloc_memory {
        println!("Error: Finch has more memory than max alloc.");
    }
}