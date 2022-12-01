use crate::lexome::Lexome;


#[derive(Debug, PartialEq, Clone)]
pub struct Finch {
    // CPU
    pub memory: Vec<Lexome>,
    pub inst_h: usize,  pub(crate) read_h: usize,      pub(crate) writ_h: usize,      pub(crate) flow_h: usize,
    pub(crate) registers: Vec<u32>,
    pub(crate) stacks: Vec<Vec<u32>>,
    pub(crate) i_buff: u32,        pub(crate) o_buff: u32,
    pub(crate) active_stack: usize,
    // Info
    pub(crate) id: u64,
    pub(crate) x_loc: usize,       pub(crate) y_loc: usize,
    pub(crate) age: u128,
    pub(crate) inputs: Vec<u32>,
    pub(crate) copy_mutation_rate: f64,
    pub(crate) max_alloc_memory: usize,
    pub(crate) copy_history: Vec<Lexome>,
    pub(crate) skip_next_non_nop_inst: bool,
}

impl Finch {
    pub(crate) fn new(id: u64, x_loc: usize, y_loc: usize) -> Finch {
        Finch {
            // CPU
            memory: vec![],
            inst_h: 0,      read_h: 0,      writ_h: 0,      flow_h: 0,
            registers: vec![0,0,0],
            stacks: vec![vec![],vec![]],
            i_buff: 0,      o_buff: 0,
            active_stack: 0,
            // Info
            id,
            x_loc,          y_loc,
            age: 0,
            inputs: vec![],
            copy_mutation_rate: 0 as f64,
            max_alloc_memory: 150,
            copy_history: vec![],
            skip_next_non_nop_inst: false,
        }
    }
    pub(crate) fn increment(&mut self) -> ReturnPacket {
        let instruction: &Lexome = &self.memory[self.inst_h];
        let mut return_packet: ReturnPacket = ReturnPacket::empty();
        let mut skip_inc: bool = false;
        if self.skip_next_non_nop_inst {
            if !is_nop(&self.memory[self.inst_h]) {
                self.inc_inst_h();
                self.skip_next_non_nop_inst = false;
                self.age += 1;
                return return_packet;
            }
        }
        match instruction {
            &Lexome::Nop  => {}
            &Lexome::NopA => {}
            &Lexome::NopB => {}
            &Lexome::NopC => {}
            &Lexome::IfNEqu => {
                let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                let register: &u32 = &self.registers[nop_to_register(&nop_ref).unwrap()];
                let complement_register: &u32 = &self.registers[
                    inc_register(nop_to_register(&nop_ref).unwrap()).unwrap()
                    ];
                if register == complement_register {
                    // skip next instruction
                    self.inc_inst_h();
                };
            }
            &Lexome::IfLess => {
                let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                let register: &u32 = &self.registers[nop_to_register(&nop_ref).unwrap()];
                let complement_register: &u32 = &self.registers[
                    inc_register(nop_to_register(&nop_ref).unwrap()).unwrap()
                    ];
                if register >= complement_register {
                    // skip next instruction
                    self.inc_inst_h();
                };
            }
            &Lexome::Pop => {
                if self.stacks[self.active_stack].len() > 0 {
                    let pop_value: u32 = self.stacks[self.active_stack].pop().unwrap();
                    let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                    self.registers[nop_to_register(&nop_ref).unwrap()] = pop_value;
                }
            }
            &Lexome::Push => {
                let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                self.stacks[self.active_stack]
                    .push(self.registers[nop_to_register(&nop_ref).unwrap()]);
            }
            &Lexome::SwapStk => {
                if self.active_stack == 1 {self.active_stack = 0}
                else {self.active_stack = 1}
            }
            &Lexome::Swap => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                let nop_complement: Lexome = inc_nop(&nop_ref).unwrap();
                let register_content: u32 = self
                    .registers[nop_to_register(&nop_ref).unwrap()]
                    .clone();
                let complement_content: u32 = self
                    .registers[nop_to_register(&nop_complement).unwrap()]
                    .clone();
                self.registers[nop_to_register(&nop_ref).unwrap()] = complement_content;
                self.registers[nop_to_register(&nop_complement).unwrap()] = register_content;
            }
            &Lexome::ShiftR => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] >> 1;
            }
            &Lexome::ShiftL => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] << 1;
            }
            &Lexome::Inc => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] + 1;
            }
            &Lexome::Dec => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&nop_ref).unwrap()] = self
                    .registers[nop_to_register(&nop_ref).unwrap()] - 1;
            }
            &Lexome::Add => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = b_value + c_value;
            }
            &Lexome::Sub => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = b_value - c_value;
            }
            &Lexome::Nand => {
                let b_value: &u32 = &self.registers[1];
                let c_value: &u32 = &self.registers[2];
                let target_reg_nop: Lexome = modify_nop(self, Lexome::NopB);
                self.registers[nop_to_register(&target_reg_nop).unwrap()] = !(b_value & c_value);
            }
            &Lexome::IO => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopB);
                return_packet.output = Some(self
                    .registers[nop_to_register(&nop_ref).unwrap()]
                );
                if self.inputs.len() > 0 {
                    self.registers[nop_to_register(&nop_ref).unwrap()] = self.inputs.pop().unwrap();
                }
            }
            &Lexome::HAlloc => {
                if self.max_alloc_memory > self.memory.len() {
                    let original_memory_size: usize = self.memory.len();
                    self.memory.append(&mut vec![Lexome::Nop; self.max_alloc_memory - self.memory.len()]);
                    self.registers[0] = original_memory_size as u32;
                }
            }
            &Lexome::HDivide => {
                if self.writ_h > self.read_h  && self.read_h > 0 {
                    let original_memory: &[Lexome] = &self.memory[0..self.read_h];
                    let offspring_memory: &[Lexome] = &self.memory[self.read_h..self.writ_h];
                    let mut offspring: Finch = Finch::new(0, 0,0);
                    offspring.memory = Vec::from(offspring_memory);
                    self.memory = Vec::from(original_memory);
                    self.read_h = 0;
                    self.writ_h = 0;
                    self.flow_h = 0;
                    return_packet.return_finch = Some(offspring);
                }
            ;}
            &Lexome::HCopy => {
                // TODO: Implement Copy Mutation
                self.memory[self.writ_h] = self.memory[self.read_h].clone();
                self.copy_history.push(self.memory[self.read_h]);
                self.read_h = inc_h_non_mut(self.memory.len(),self.read_h,1);
                self.writ_h = inc_h_non_mut(self.memory.len(),self.writ_h,1);
            }
            // Jesus H. Christ this is a mess.
            // This is the string search problem but for circular strings.
            &Lexome::HSearch => {
                let mut nop_label: Vec<Lexome> = read_nop_label(&self.memory,self.inst_h);
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
                    let mut search_mem: Vec<Lexome> = self
                        .memory[self.inst_h + nop_label.len() + 1..self.memory.len()]
                        .to_vec();
                    search_mem.append(&mut self.memory[0..self.inst_h].to_vec());
                    let mut index: usize = 0;
                    while index < search_mem.len() - nop_label.len() + 1{
                        let test_vec: Vec<Lexome> = search_mem[index..index + nop_label.len()]
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
                        println!("abs_pos: {}, nop_label.len(): {}",abs_pos, nop_label.len());
                        self.flow_h =  inc_h_non_mut(
                            self.memory.len(),
                            abs_pos,
                            (nop_label.len()) as u8
                        );
                    }
                }
            }
            &Lexome::MovHead => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopA);
                match nop_ref {
                    Lexome::NopA => {self.inst_h = self.flow_h; skip_inc = true;},
                    Lexome::NopB => {self.read_h = self.flow_h;},
                    Lexome::NopC => {self.writ_h = self.flow_h;},
                    _ => {},
                }

            }
            &Lexome::JmpHead => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopA);
                let c_val: u32 = self.registers[2];
                match nop_ref {
                    Lexome::NopA => {
                        self.inst_h = inc_h_non_mut(self.memory.len(), self.inst_h, c_val as u8);
                        skip_inc = true;
                    },
                    Lexome::NopB => {
                        self.read_h = inc_h_non_mut(self.memory.len(), self.read_h, c_val as u8);
                    },
                    Lexome::NopC => {
                        self.writ_h = inc_h_non_mut(self.memory.len(), self.writ_h, c_val as u8);
                    },
                    _ => {},
                }
            }
            &Lexome::GetHead => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopA);
                match nop_ref {
                    Lexome::NopA => {self.registers[2] = self.inst_h as u32; },
                    Lexome::NopB => {self.registers[2] = self.read_h as u32; },
                    Lexome::NopC => {self.registers[2] = self.writ_h as u32; },
                    _ => {},
                }}
            &Lexome::IfLabel => {
                let mut nop_label: Vec<Lexome> = read_nop_label(&self.memory, self.inst_h);
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
            &Lexome::SetFlow => {
                let nop_ref: Lexome = modify_nop(self, Lexome::NopC);
                let reg_val: u32 = self.registers[nop_to_register(&nop_ref).unwrap()];
                self.flow_h = reg_val as usize % self.memory.len();
            }
        };
        if !skip_inc {
            self.inc_inst_h();
        }
        self.age += 1;
        return_packet
    }
    pub(crate) fn inc_inst_h(&mut self) {
        self.inst_h = inc_h_non_mut(self.memory.len(), self.inst_h,1)
    }
}

pub fn read_nop_label(memory: &Vec<Lexome>, current_pos: usize) -> Vec<Lexome> {
    let mut flag: bool = true;
    let mut pos: usize = current_pos;
    let mut label_builder: Vec<Lexome> = vec![];
    while flag {
        pos = inc_h_non_mut(memory.len(),pos,1);
        if is_nop(&memory[pos]) {
            label_builder.push(*&memory[pos]);
        }
        else {flag = false}
    }
    label_builder
}

fn modify_nop(finch: &Finch, default_nop: Lexome) -> Lexome{
    let next_inst: Lexome = finch.memory[
        inc_h_non_mut(finch.memory.len(), finch.inst_h,1)
        ].clone();
    if is_nop(&next_inst) { next_inst }
    else { default_nop }
}

pub fn inc_h_non_mut(length: usize, current_h: usize, repeat: u8) -> usize {
    let mut pos: usize = current_h;
    for _ in 0..repeat {
        if pos + 1 >= length {pos = 0}
        else {pos = pos + 1}
    }
    pos
}

fn is_nop(nop: &Lexome) -> bool {
    nop == &Lexome::NopA || nop == &Lexome::NopB || nop == &Lexome::NopC
}

fn inc_nop(nop: &Lexome) -> Option<Lexome> {
    return match nop {
        Lexome::NopA => Some(Lexome::NopB),
        Lexome::NopB => Some(Lexome::NopC),
        Lexome::NopC => Some(Lexome::NopA),
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

fn nop_to_register(nop: &Lexome) -> Option<usize> {
    return match nop {
        Lexome::NopA => Some(0),
        Lexome::NopB => Some(1),
        Lexome::NopC => Some(2),
        _ => None,
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

// TODO Phase out eventually.
pub fn dummy_memory() -> Vec<Lexome> {
    vec![
        Lexome::HAlloc,
        Lexome::HSearch,
        Lexome::NopC,
        Lexome::NopA,
        Lexome::MovHead,
        Lexome::NopC,
        // Copy Loop
        Lexome::HSearch,

        Lexome::HCopy,
        Lexome::IfLabel,
        Lexome::NopC,
        Lexome::NopA,
        Lexome::HDivide,
        Lexome::MovHead,
        Lexome::NopA,
        Lexome::NopB
    ]
}
