use rand::rngs::ThreadRng;
use crate::Finch;
use crate::mutations::{copy_mutation, insertion_deletion_mutation, point_mutation};

pub fn read_nop_label(memory: &Vec<Instructions>, current_pos: usize) -> Vec<Instructions> {
    let mut flag: bool = true;
    let mut pos: usize = current_pos;
    let mut label_builder: Vec<Instructions> = vec![];
    while flag {
        pos = inc_h_non_mut(memory.len(),pos,1);
        if is_nop(&memory[pos]) {
            label_builder.push(*&memory[pos]);
        }
        else {flag = false}
    }
    label_builder
}

fn modify_nop(finch: &Finch, default_nop: Instructions) -> Instructions {
    let next_inst: Instructions = finch.memory[
        inc_h_non_mut(finch.memory.len(), finch.inst_h,1)
        ].clone();
    if is_nop(&next_inst) { next_inst }
    else { default_nop }
}

pub(crate) fn inc_h_non_mut(length: usize, current_h: usize, repeat: u8) -> usize {
    let mut pos: usize = current_h;
    for _ in 0..repeat {
        if pos + 1 >= length {pos = 0}
        else {pos = pos + 1}
    }
    pos
}

fn is_nop(nop: &Instructions) -> bool {
    nop == &Instructions::NopA || nop == &Instructions::NopB || nop == &Instructions::NopC
}

fn inc_nop(nop: &Instructions) -> Option<Instructions> {
    return match nop {
        Instructions::NopA => Some(Instructions::NopB),
        Instructions::NopB => Some(Instructions::NopC),
        Instructions::NopC => Some(Instructions::NopA),
        _ => None
    }
}

fn inc_register(index: usize) -> Option<usize> {
    return match index {
        0 => Some(1),
        1 => Some(2),
        2 => Some(3),
        _ => None
    }
}

fn nop_to_register(nop: &Instructions) -> Option<usize> {
    return match nop {
        Instructions::NopA => Some(0),
        Instructions::NopB => Some(1),
        Instructions::NopC => Some(2),
        _ => None,
    }
}

impl Finch {
    pub(crate) fn clock_cycle_execute(&mut self, thread_rng: &mut ThreadRng) -> ReturnPacket {
        let instruction: &Instructions = &self.memory[self.inst_h];
        let mut return_packet: ReturnPacket = ReturnPacket::empty();
        let mut skip_inc: bool = false;
        if self.skip_next_non_nop_inst {
            if !is_nop(&self.memory[self.inst_h]) {
                self.inc_inst_h();
                self.skip_next_non_nop_inst = false;
                self.age += 1;
                self.memory = point_mutation(
                    &self.memory,
                    self.point_mutation_rate,
                    thread_rng
                );
                return return_packet;
            }
        }
        match instruction {
            &Instructions::Nop  => {}
            &Instructions::NopA => {}
            &Instructions::NopB => {}
            &Instructions::NopC => {}
            &Instructions::IfNEqu => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                let register: &u32 = &self.registers[nop_to_register(&nop_ref).unwrap()];
                let complement_register: &u32 = &self.registers[
                    inc_register(nop_to_register(&nop_ref).unwrap()).unwrap()
                    ];
                if register == complement_register {
                    // skip next instruction
                    self.inc_inst_h();
                };
            }
            &Instructions::IfLess => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                let register: &u32 = &self.registers[nop_to_register(&nop_ref).unwrap()];
                let complement_register: &u32 = &self.registers[
                    inc_register(nop_to_register(&nop_ref).unwrap()).unwrap()
                    ];
                if register >= complement_register {
                    // skip next instruction
                    self.inc_inst_h();
                };
            }
            &Instructions::Pop => {
                if self.stacks[self.active_stack].len() > 0 {
                    let pop_value: u32 = self.stacks[self.active_stack].pop().unwrap();
                    let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                    self.registers[nop_to_register(&nop_ref).unwrap()] = pop_value;
                }
            }
            &Instructions::Push => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                self.stacks[self.active_stack]
                    .push(self.registers[nop_to_register(&nop_ref).unwrap()]);
            }
            &Instructions::SwapStk => {
                if self.active_stack == 1 {self.active_stack = 0}
                else {self.active_stack = 1}
            }
            &Instructions::Swap => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                let nop_complement: Instructions = inc_nop(&nop_ref).unwrap();
                let register_content: u32 = self
                    .registers[nop_to_register(&nop_ref).unwrap()]
                    .clone();
                let complement_content: u32 = self
                    .registers[nop_to_register(&nop_complement).unwrap()]
                    .clone();
                self.registers[nop_to_register(&nop_ref).unwrap()] = complement_content;
                self.registers[nop_to_register(&nop_complement).unwrap()] = register_content;
            }
            &Instructions::ShiftR => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] >> 1;
            }
            &Instructions::ShiftL => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] << 1;
            }
            &Instructions::Inc => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] + 1;
            }
            &Instructions::Dec => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] - 1;
            }
            &Instructions::Add => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = b_value + c_value;
            }
            &Instructions::Sub => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = b_value - c_value;
            }
            &Instructions::Nand => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Instructions = modify_nop(self, Instructions::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = !(b_value & c_value);
            }
            &Instructions::IO => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopB);
                return_packet.output = Some(self
                    .registers[nop_to_register(&nop_ref).unwrap()]
                );
                if self.inputs.len() > 0 {
                    self.registers[nop_to_register(&nop_ref).unwrap()] = self.inputs.pop().unwrap();
                }
            }
            &Instructions::HAlloc => {
                if self.max_alloc_memory > self.memory.len() {
                    let original_memory_size: usize = self.memory.len();
                    self.memory.append(&mut vec![Instructions::Nop; self.max_alloc_memory - self.memory.len()]);
                    self.registers[0] = original_memory_size as u32;
                }
            }
            &Instructions::HDivide => {
                if self.writ_h > self.read_h  && self.read_h > 0 {
                    let original_memory: &[Instructions] = &self.memory[0..self.read_h];
                    let offspring_memory: &[Instructions] = &self.memory[self.read_h..self.writ_h];
                    let mut offspring: Finch = Finch::new(0, 0,0);
                    offspring.memory = Vec::from(offspring_memory);
                    self.memory = Vec::from(original_memory);
                    self.read_h = 0;
                    self.writ_h = 0;
                    self.flow_h = 0;
                    // Insertion and deletion mutation.
                    offspring.memory = insertion_deletion_mutation(
                        offspring.memory,
                        offspring.divide_mutation_rate,
                        thread_rng
                    );
                    println!("{:?}", offspring.memory);
                    return_packet.return_finch = Some(offspring);

                }

            ;}
            &Instructions::HCopy => {
                self.pre_mut_copy_history.push(self.memory[self.read_h]);
                // Copy Mutation
                let post_mut_inst: Instructions = copy_mutation(
                    self.memory[self.read_h],
                    self.copy_mutation_rate,
                    thread_rng
                );
                self.copy_history.push(self.memory[self.read_h]);
                self.memory[self.writ_h] = post_mut_inst;
                self.read_h = inc_h_non_mut(self.memory.len(),self.read_h,1);
                self.writ_h = inc_h_non_mut(self.memory.len(),self.writ_h,1);
            }
            // Jesus H. Christ this is a mess.
            // This is the string search problem for circular strings with naive implementation.
            // Should only be O(N), and mem is capped at 100.
            &Instructions::HSearch => {
                let mut nop_label: Vec<Instructions> = read_nop_label(&self.memory, self.inst_h);
                for i in 0..nop_label.len() {
                    nop_label[i] = inc_nop(&nop_label[i]).unwrap();
                }
                if nop_label.len() == 0 {
                    self.registers[1] = 0;
                    self.registers[2] = 0;
                    self.flow_h = inc_h_non_mut(self.memory.len(),self.inst_h,1);
                }
                else {
                    let mut present_flag: bool = false;
                    let mut search_mem: Vec<Instructions> = self
                        .memory[self.inst_h + nop_label.len() + 1..self.memory.len()]
                        .to_vec();
                    search_mem.append(&mut self.memory[0..self.inst_h].to_vec());
                    let mut index: usize = 0;
                    while index < search_mem.len() - nop_label.len() + 1{
                        let test_vec: Vec<Instructions> = search_mem[index..index + nop_label.len()]
                            .to_vec();
                        if test_vec == nop_label {
                            present_flag = true;
                            break;
                        }
                        else {
                            index += 1;
                        }
                    }
                    if present_flag {
                        let mut abs_pos: usize = 0;
                        // fix
                        if index > self.memory.len() - (self.inst_h + nop_label.len()) - 1 {
                            abs_pos = index - (self.memory.len() - self.inst_h - nop_label.len()) + 1;
                        }

                        if index < self.memory.len() - (self.inst_h + nop_label.len()) - 1 {
                            abs_pos = index + (self.inst_h + nop_label.len() + 1 );
                        }
                        if index == self.memory.len() - (self.inst_h + nop_label.len()) - 1 {
                            abs_pos = 0;
                        }
                        self.registers[1] = (index + nop_label.len() + 1) as u32;
                        //self.registers[1] = (abs_pos as i32 - self.inst_h as i32).abs() as u32;
                        self.registers[2] = nop_label.len() as u32;
                        self.flow_h =  inc_h_non_mut(
                            self.memory.len(),
                            abs_pos,
                            (nop_label.len()) as u8
                        );
                    }
                }
            }
            &Instructions::MovHead => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopA);
                match nop_ref {
                    Instructions::NopA => {self.inst_h = self.flow_h; skip_inc = true;},
                    Instructions::NopB => {self.read_h = self.flow_h;},
                    Instructions::NopC => {self.writ_h = self.flow_h;},
                    _ => {},
                }

            }
            &Instructions::JmpHead => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopA);
                let c_val: u32 = self.registers[2];
                match nop_ref {
                    Instructions::NopA => {
                        self.inst_h = inc_h_non_mut(self.memory.len(), self.inst_h, c_val as u8);
                        skip_inc = true;
                    },
                    Instructions::NopB => {
                        self.read_h = inc_h_non_mut(self.memory.len(), self.read_h, c_val as u8);
                    },
                    Instructions::NopC => {
                        self.writ_h = inc_h_non_mut(self.memory.len(), self.writ_h, c_val as u8);
                    },
                    _ => {},
                }
            }
            &Instructions::GetHead => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopA);
                match nop_ref {
                    Instructions::NopA => {self.registers[2] = self.inst_h as u32; },
                    Instructions::NopB => {self.registers[2] = self.read_h as u32; },
                    Instructions::NopC => {self.registers[2] = self.writ_h as u32; },
                    _ => {},
                }}
            &Instructions::IfLabel => {
                let mut nop_label: Vec<Instructions> = read_nop_label(&self.memory, self.inst_h);
                for i in 0..nop_label.len() {
                    nop_label[i] = inc_nop(&nop_label[i]).unwrap();
                }
                let hist_len: usize = self.copy_history.len();
                let mut skip_flag: bool = true;
                if nop_label.len() <= hist_len {
                    if nop_label[0..nop_label.len()] == self.copy_history[hist_len - nop_label.len() .. hist_len] {
                        skip_flag = false;
                    }
                }
                if skip_flag {
                    self.skip_next_non_nop_inst = true;
                }
            }
            &Instructions::SetFlow => {
                let nop_ref: Instructions = modify_nop(self, Instructions::NopC);
                let reg_val: u32 = self.registers[nop_to_register(&nop_ref).unwrap()];
                self.flow_h = reg_val as usize % self.memory.len();
            }
        };
        if !skip_inc {
            self.inc_inst_h();
        }
        self.age += 1;
        // Point Mutations
        return_packet
    }
    pub(crate) fn inc_inst_h(&mut self) {
        self.inst_h = inc_h_non_mut(self.memory.len(), self.inst_h,1)
    }
}


#[derive(Debug)]
pub struct ReturnPacket {
    pub(crate) output: Option<u32>,
    pub(crate) return_finch: Option<Finch>,
}

impl ReturnPacket {
    fn empty() -> ReturnPacket {
        ReturnPacket {
            output: None,
            return_finch: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instructions {
    Nop, NopA, NopB, NopC, IfNEqu, IfLess, Pop, Push, SwapStk, Swap, ShiftR, ShiftL, Inc, Dec, Add,
    Sub, Nand, IO, HAlloc, HDivide, HCopy, HSearch, MovHead, JmpHead, GetHead, IfLabel, SetFlow,
}

pub fn possible_instructions() -> Vec<Instructions> {
    vec![
        Instructions::NopA, Instructions::NopB, Instructions::NopC, Instructions::IfNEqu,
        Instructions::IfLess, Instructions::Pop, Instructions::Push, Instructions::SwapStk, Instructions::Swap,
        Instructions::ShiftR, Instructions::ShiftL, Instructions::Inc, Instructions::Dec, Instructions::Add,
        Instructions::Sub, Instructions::Nand, Instructions::IO, Instructions::HAlloc, Instructions::HDivide,
        Instructions::HCopy, Instructions::HSearch, Instructions::MovHead, Instructions::JmpHead,
        Instructions::GetHead, Instructions::IfLabel, Instructions::SetFlow
    ]
}

impl Instructions {
    // TODO Lexome reading from files
}

pub fn dummy_memory() -> Vec<Instructions> {
    vec![
        Instructions::HAlloc,
        Instructions::HSearch,
        Instructions::NopC,
        Instructions::NopA,
        Instructions::MovHead,
        Instructions::NopC,
        // Copy Loop
        Instructions::HSearch,
        Instructions::HCopy,
        Instructions::IfLabel,
        Instructions::NopC,
        Instructions::NopA,
        Instructions::HDivide,
        Instructions::MovHead,
        Instructions::NopA,
        Instructions::NopB
    ]
}

