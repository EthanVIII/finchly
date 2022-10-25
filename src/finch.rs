use crate::lexome::Lexome;

#[derive(Debug, PartialEq, Clone)]
pub struct Finch {
    // CPU
    pub lexome: Vec<Lexome>,
    pub inst_h: usize,  pub(crate) read_h: usize,      pub(crate) writ_h: usize,      pub(crate) flow_h: usize,
    pub(crate) regi_1: u32,        pub(crate) regi_2: u32,        pub(crate) regi_3: u32,
    pub(crate) stac_1: Vec<u32>,   pub(crate) stac_2: Vec<u32>,
    pub(crate) i_buff: u32,        pub(crate) o_buff: u32,
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
            regi_1: 0,      regi_2: 0,      regi_3: 0,
            stac_1: vec![], stac_2: vec![],
            i_buff: 0,      o_buff: 0,
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
            &Lexome::IfNEqu => { println!("IfNEq"); None }
            &Lexome::IfLess => { println!("IfLess"); None }
            &Lexome::Pop => { println!("Pop"); None }
            &Lexome::Push => { println!("Push"); None }
            &Lexome::SwapStk => { println!("SwapStk"); None }
            &Lexome::Swap => { println!("Swap"); None }
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
        if self.inst_h + 1 == self.lexome.len() {
            self.inst_h = 0;
        }
        else {
            self.inst_h += 1;
        }
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
