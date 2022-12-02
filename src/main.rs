mod finch;
mod tests;
mod instructions;

use instructions::{dummy_memory, Instructions, ReturnPacket};
use crate::finch::Finch;

fn main() {
    let mut finch: Finch = Finch::new(0,0,0);
    finch.memory = dummy_memory();
    for i in 0..100 {
        println!("{:?}",finch.memory[finch.inst_h]);
        let return_packet: ReturnPacket = finch.clock_cycle_execute();
    }
}

struct Aviary {
    max_pop: u32,
    dim: (u32,u32),
    finches: Vec<Finch>,
}

enum LoggingType {
    Full,
    Partial,
    None,
}