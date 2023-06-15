use rand::{Rng};
use rand::rngs::ThreadRng;
use crate::Instructions;
use crate::instructions::{possible_instructions};

pub(crate) fn point_mutation(
    memory: &Vec<Instructions>,
    mutation_probability: f64,
    rng_thread: &mut ThreadRng
) -> Vec<Instructions> {
    let mutation_threshold: f64 = rng_thread.gen();
    let mut copy_mutated: Vec<Instructions> = memory[0..memory.len()].to_vec();
    if mutation_threshold <= mutation_probability {
        let mutation_position: usize = rng_thread.gen_range(0..memory.len());
        copy_mutated[mutation_position] = sample_excluding(
            copy_mutated[mutation_position],
            rng_thread
        ).unwrap();
    }
    copy_mutated
}

pub(crate) fn insertion_deletion_mutation(
    mut memory: Vec<Instructions>,
    mutation_probability: f64,
    rng_thread: &mut ThreadRng
) -> Vec<Instructions> {
    let mutation_threshold: f64 = rng_thread.gen();
    if mutation_threshold <= mutation_probability {
        let is_insertion: bool = rng_thread.gen_bool(0.5);
        let mutation_position: usize = rng_thread.gen_range(0..memory.len());
        if is_insertion {
            memory.insert(mutation_position, sample_inst(rng_thread));
        }
        else {
            memory.remove(mutation_position); // Else Deletion happens
        }
    }
    memory
}


pub(crate) fn copy_mutation(
    instruction: Instructions,
    mutation_probability: f64,
    rng_thread: &mut ThreadRng,
) -> Instructions {
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

fn sample_excluding(instruction: Instructions, rng_thread: &mut ThreadRng) -> Option<Instructions> {
    let mut instructions: Vec<Instructions> = possible_instructions();
    let inst_pos: Option<usize> = instructions
        .iter()
        .position(|&x| x == instruction);
    return match inst_pos {
        // Tried to copy unsampled command.
        None => {None}
        Some(x) => {
            instructions.remove(x);
            let sampling_index: usize = rng_thread.gen_range(0..instructions.len());
            Some(instructions[sampling_index])
        }
    }
}

fn sample_inst(rng_thread: &mut ThreadRng) -> Instructions {
    let instructions: Vec<Instructions> = possible_instructions();
    let sample_value: usize = rng_thread.gen_range(0..instructions.len());
    instructions[sample_value]
}