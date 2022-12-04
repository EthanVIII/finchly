use rand::Rng;
use rand::rngs::ThreadRng;
use crate::Instructions;
use crate::instructions::{possible_instructions};

pub(crate) fn copy_mutation(instruction: Instructions, mutation_probability: f64) -> Instructions {
    let mut rng_thread: ThreadRng = rand::thread_rng();
    let mutation_threshold: f64 = rng_thread.gen();
    if mutation_threshold <= mutation_probability {
        let opt_mut: Option<Instructions> = sample_excluding(instruction, rng_thread);
        return match opt_mut {
            // If it is unsampled, return inst and println for debug.
            None => {
                println!("Debug Error: Attempted to sample unsampled inst.");
                instruction
            }
            Some(opt) => {opt}
        }
    }
    instruction
}

fn sample_excluding(instruction: Instructions, mut rng_thread: ThreadRng) -> Option<Instructions> {
    let mut instructions: Vec<Instructions> = possible_instructions();
    let inst_pos: Option<usize> = instructions
        .iter()
        .position(|&x| x == instruction);
    return match inst_pos {
        // Tried to copy unsampleable command.
        None => {None}
        Some(x) => {
            instructions.remove(x);
            let sampling_index: usize = rng_thread.gen_range(0..instructions.len());
            Some(instructions[sampling_index])
        }
    }
}