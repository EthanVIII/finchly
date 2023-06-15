mod finch;
mod tests;
mod instructions;
mod mutations;

use instructions::{dummy_memory, Instructions, ReturnPacket};
use crate::finch::Finch;
use rand::rngs::ThreadRng;


fn main() {
    let mut aviary: Aviary = Aviary {
        dim: (30,30),
        finches: vec![],
    };
    let mut finch: Finch = Finch::new(0,0,0);
    let mut rng_thread: ThreadRng = rand::thread_rng();
    finch.memory = dummy_memory();
    for i in 0..100 {
        println!("{:?}",finch.memory[finch.inst_h]);
        let return_packet: ReturnPacket = finch.clock_cycle_execute(&mut rng_thread);
    }
}

// Input and Output
struct Aviary {
    dim: (u32,u32),
    finches: Vec<Finch>,
}

enum LoggingType {
    Full,
    Partial,
    None,
}

