#[cfg(test)]
mod tests {
    use crate::Finch;
    use crate::finch;
    use crate::lexome;
    use crate::lexome::Lexome;
    use crate::lexome::Lexome::{IfNEqu, NopA, NopB, NopC};

    #[test]
    fn increment() {
        let mut new_finch: Finch = Finch::new(0,0,0);
        new_finch.memory = finch::dummy_memory();
        new_finch.increment();
        assert_eq!(new_finch.inst_h,1);
    }

    // NopA, NopB, NopC,
    #[test]
    fn nops() {
        let mut new_finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![NopA,NopB,NopC,NopC];
        new_finch.memory = test_lexome.clone();

        let mut comparing_finch: Finch = Finch {
            memory: test_lexome.clone(),
            inst_h: 3,
            read_h: 0,
            writ_h: 0,
            flow_h: 0,
            regi_1: 0,
            regi_2: 0,
            regi_3: 0,
            stac_1: vec![],
            stac_2: vec![],
            i_buff: 0,
            o_buff: 0,
            id: 0,
            x_loc: 0,
            y_loc: 0,
            age: 3,
            inputs: vec![]
        };
        new_finch.increment();
        new_finch.increment();
        new_finch.increment();
        assert_eq!(comparing_finch,new_finch);
    }

    // IfNEqu  	Execute next instruction only-if ?BX? does not equal its complement
    #[test]
    fn if_n_equ() {
        let mut new_finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![IfNEqu,NopB,NopC,NopC];
        new_finch.memory = test_lexome.clone();
        let mut comp_finch: Finch = Finch {
            memory: test_lexome.clone(),
            inst_h: 1,
            read_h: 0,
            writ_h: 0,
            flow_h: 0,
            regi_1: 0,
            regi_2: 0,
            regi_3: 0,
            stac_1: vec![],
            stac_2: vec![],
            i_buff: 0,
            o_buff: 0,
            id: 0,
            x_loc: 0,
            y_loc: 0,
            age: 1,
            inputs: vec![]
        };
        assert_eq!(new_finch,comp_finch);

        let mut new_finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![IfNEqu,NopB,NopC,NopC];
        new_finch.regi_3 = 1;
        new_finch.memory = test_lexome.clone();
        let mut comp_finch: Finch = Finch {
            memory: test_lexome.clone(),
            inst_h: 3,
            read_h: 0,
            writ_h: 0,
            flow_h: 0,
            regi_1: 0,
            regi_2: 0,
            regi_3: 1,
            stac_1: vec![],
            stac_2: vec![],
            i_buff: 0,
            o_buff: 0,
            id: 0,
            x_loc: 0,
            y_loc: 0,
            age: 1,
            inputs: vec![]
        };
        new_finch.increment();
        assert_eq!(comp_finch,new_finch);

    }

    // IfLess,
    // Pop,
    // Push,
    // SwapStk,
    // Swap,
    // ShiftR,
    // ShiftL,
    // Inc,
    // Dec,
    // Add,
    // Sub,
    // Nand,
    // IO,
    // HAlloc,
    // HDivide,
    // HCopy,
    // HSearch,
    // MovHead,
    // JmpHead,
    // GetHead,
    // IfLabel,
    // SetFlow,

}
