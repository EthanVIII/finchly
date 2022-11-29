mod finch;
mod lexome;
mod tests;

use crate::finch::{Finch, ReturnPacket};
use crate::lexome::Lexome;

fn main() {
    let mut finch: Finch = Finch::new(0,0,0);
    finch.lexome = finch::dummy_lexome();
    for i in 0..100 {
        let return_packet: ReturnPacket = finch.increment();
    }
}
