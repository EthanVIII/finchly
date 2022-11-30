mod finch;
mod lexome;
mod tests;

use crate::finch::{Finch, ReturnPacket};
use crate::lexome::Lexome;

fn main() {
    let mut finch: Finch = Finch::new(0,0,0);
    finch.memory = finch::dummy_memory();
    for i in 0..100 {
        let return_packet: ReturnPacket = finch.increment();
    }
}
