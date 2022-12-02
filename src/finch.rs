use crate::instructions::Instructions;

#[derive(Debug, PartialEq, Clone)]
pub struct Finch {
    // CPU
    pub memory: Vec<Instructions>,
    pub inst_h: usize,
    pub(crate) read_h: usize,
    pub(crate) writ_h: usize,
    pub(crate) flow_h: usize,
    pub(crate) registers: Vec<u32>,
    pub(crate) stacks: Vec<Vec<u32>>,
    pub(crate) i_buff: u32,        pub(crate) o_buff: u32,
    pub(crate) active_stack: usize,
    // Info
    pub(crate) id: u64,
    pub(crate) x_loc: usize,
    pub(crate) y_loc: usize,
    pub(crate) age: u128,
    pub(crate) inputs: Vec<u32>,
    pub(crate) copy_mutation_rate: f64,
    pub(crate) max_alloc_memory: usize,
    pub(crate) copy_history: Vec<Instructions>,
    pub(crate) pre_mut_copy_history: Vec<Instructions>,
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
            pre_mut_copy_history: vec![],
            skip_next_non_nop_inst: false,
        }
    }
}
