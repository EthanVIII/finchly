use crate::lexome::Lexome;
use crate::lexome::Lexome::{NopA, NopB, NopC};

#[derive(Debug, PartialEq, Clone)]
pub struct Finch {
    // CPU
    pub lexome: Vec<Lexome>,
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
}

impl Finch {
    pub(crate) fn new(id: u64, x_loc: usize, y_loc: usize) -> Finch {
        Finch {
            // CPU
            lexome: vec![],
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
        }
    }
    pub(crate) fn increment(&mut self) -> ReturnPacket {
        self.age += 1;
        // execute order
        let instruction: &Lexome = &self.lexome[self.inst_h];
        let return_finch: Option<Finch> = match instruction {
            &Lexome::NopA => { None }
            &Lexome::NopB => { None }
            &Lexome::NopC => { None }
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
                None
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
                None
            }
            &Lexome::Pop => {
                if self.stacks[self.active_stack].len() > 0 {
                    let pop_value: u32 = self.stacks[self.active_stack].pop().unwrap();
                    let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                    self.registers[nop_to_register(&nop_ref).unwrap()] = pop_value;
                }
                None
            }
            &Lexome::Push => {
                let nop_ref: Lexome = modify_nop(self,Lexome::NopB);
                self.stacks[self.active_stack]
                    .push(self.registers[nop_to_register(&nop_ref).unwrap()]);
                None
            }
            &Lexome::SwapStk => {
                if self.active_stack == 1 {self.active_stack = 0}
                else {self.active_stack = 1}
                None
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
                None
            }
            &Lexome::ShiftR => { println!("ShiftR"); None }
            &Lexome::ShiftL => { println!("ShiftL"); None }
            &Lexome::Inc => { println!("Inc"); None }
            &Lexome::Dec => { println!("Dec"); None }
            &Lexome::Add => { println!("Add"); None }
            &Lexome::Sub => { println!("Sub"); None }
            &Lexome::Nand => { println!("Nand"); None }
            &Lexome::IO => { println!("IO"); None }
            &Lexome::HAlloc => { println!("HAlloc"); None }
            &Lexome::HDivide => { println!("HDivide"); None }
            &Lexome::HCopy => { println!("HCopy"); None }
            &Lexome::HSearch => { println!("HSearch"); None }
            &Lexome::MovHead => { println!("MovHead"); None }
            &Lexome::JmpHead => { println!("JmpHead"); None }
            &Lexome::GetHead => { println!("GetHead"); None }
            &Lexome::IfLabel => { println!("IfLabel"); None }
            &Lexome::SetFlow => { println!("SetFlow"); None }
        };
        self.inc_inst_h();
        ReturnPacket::empty()
    }
    pub(crate) fn inc_inst_h(&mut self) {
        self.inst_h = inc_h_non_mut(self.lexome.len(), self.inst_h)
    }
}

fn modify_nop(finch: &mut Finch, default_nop: Lexome) -> Lexome{
    let next_inst: Lexome = finch.lexome[
        inc_h_non_mut(finch.lexome.len(),finch.inst_h)
        ].clone();
    if is_nop(&next_inst) { next_inst }
    else { default_nop }
}

fn inc_h_non_mut(length: usize, current_h: usize) -> usize {
    if current_h + 1 == length {0}
    else {current_h + 1}
}

fn is_nop(nop: &Lexome) -> bool {
    nop == &NopA || nop == &NopB || nop == &NopC
}

fn inc_nop(nop: &Lexome) -> Option<Lexome> {
    return match nop {
        NopA => Some(NopB),
        NopB => Some(NopC),
        NopC => Some(NopA),
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
        NopA => Some(0),
        NopB => Some(1),
        NopC => Some(2),
        _ => None,
    }
}


pub struct ReturnPacket {
    return_finch: Option<Finch>,
    alloc_request: Option<usize>,
}
impl ReturnPacket {
    fn empty() -> ReturnPacket {
        ReturnPacket {
            return_finch: None,
            alloc_request: None
        }
    }
    fn return_packet(return_finch: Option<Finch>, alloc_request: Option<usize>) -> ReturnPacket {
        ReturnPacket {
            return_finch,
            alloc_request,
        }
    }
}

// TODO Phase out eventually.
pub fn dummy_lexome() -> Vec<Lexome> {
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
