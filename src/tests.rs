#[cfg(test)]
mod instruction_tests {
    use crate::Finch;
    use crate::finch;
    use crate::finch::{dummy_memory, read_nop_label};
    use crate::lexome;
    use crate::Lexome::{HAlloc, HSearch, MovHead, Nop};
    use crate::lexome::Lexome;
    use crate::lexome::Lexome::{IfNEqu, NopA, NopB, NopC};

    #[test]
    fn increment_1() {
        let mut new_finch: Finch = Finch::new(0,0,0);
        new_finch.memory = finch::dummy_memory();
        new_finch.increment();
        assert_eq!(new_finch.inst_h,1);
    }

    // NopA, NopB, NopC,
    #[test]
    fn nop_1() {
        let mut new_finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![NopA,NopB,NopC,NopC];
        new_finch.memory = test_lexome.clone();

        let mut comparing_finch: Finch = new_finch.clone();
        comparing_finch.age = 3;
        comparing_finch.inst_h = 3;

        new_finch.increment();
        new_finch.increment();
        new_finch.increment();
        assert_eq!(comparing_finch,new_finch);
    }

    // Halloc
    #[test]
    fn h_alloc_1() {
        let mut finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![HAlloc,NopC,NopA,NopC];
        // We should see the length of memory to be 150 in total
        finch.memory = test_lexome;
        finch.increment();
        assert_eq!(finch.memory.len(),150 as usize);
    }

    #[test]
    fn h_alloc_2() {
        let mut finch: Finch = Finch::new(0,0,0);
        let test_lexome: Vec<Lexome> = vec![HAlloc; 150];
        // We should see the length of memory to be 150 in total
        finch.memory = test_lexome;
        finch.increment();
        assert_eq!(finch.memory.len(),150 as usize);
    }

    // MovHead
    #[test]
    fn mov_head_1() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![MovHead];
        finch.memory.append(&mut dummy_memory());
        finch.flow_h = 10;
        finch.increment();
        assert_eq!(finch.inst_h, finch.flow_h);
    }

    #[test]
    fn mov_head_2() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![MovHead,NopB];
        finch.memory.append(&mut dummy_memory());
        finch.flow_h = 10;
        finch.increment();
        assert_eq!(finch.read_h, finch.flow_h);
    }

    #[test]
    fn mov_head_3() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![MovHead,NopC];
        finch.memory.append(&mut dummy_memory());
        finch.flow_h = 10;
        finch.increment();
        assert_eq!(finch.writ_h, finch.flow_h);
    }

    // HSearch
    #[test]
    fn h_search_1() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopB,NopC,HSearch,NopA,NopB,NopC,MovHead,Nop,Nop,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA,NopC,NopA,
                            NopB,NopC,Nop,Nop];
        finch.inst_h = 3;
        finch.increment();
        assert_eq!(finch.registers[2],3);
        assert_eq!(finch.registers[1],21);
        assert_eq!(finch.flow_h,21);

    }

    #[test]
    fn h_search_2() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![Nop,Nop,HSearch,NopA,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA];
        finch.inst_h = 2;
        finch.increment();
        assert_eq!(finch.registers[2],1);
        assert_eq!(finch.registers[1],18);
        assert_eq!(finch.flow_h,18);

    }

    #[test]
    fn h_search_3() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,HSearch,NopA,NopB,NopC,NopA,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA];
        finch.inst_h = 2;
        finch.increment();
        assert_eq!(finch.registers[2],0);
        assert_eq!(finch.registers[1],0);
        assert_eq!(finch.flow_h,0);
    }

    #[test]
    fn h_search_4() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,HSearch,NopA,NopB,NopC,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA];
        finch.inst_h = 3;
        finch.increment();
        assert_eq!(finch.registers[2],4);
        assert_eq!(finch.registers[1],22);
        assert_eq!(finch.flow_h,22);
    }

    #[test]
    fn h_search_5() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,Nop,Nop,HSearch,NopA,NopB,NopC,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA];
        finch.inst_h = 5;
        finch.increment();
        assert_eq!(finch.registers[2],4);
        assert_eq!(finch.registers[1],24);
        assert_eq!(finch.flow_h,24);
    }

    #[test]
    fn h_search_6() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![MovHead,NopB,NopC,NopA,Nop,Nop,HSearch,NopB,NopC,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop];
        finch.inst_h = 6;
        finch.increment();
        assert_eq!(finch.registers[2],3);
        assert_eq!(finch.registers[1],1);
        assert_eq!(finch.flow_h,1);
    }

    #[test]
    fn h_search_7() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,Nop,Nop,HSearch,NopB,NopC,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop];
        finch.inst_h = 5;
        finch.increment();
        assert_eq!(finch.registers[2],3);
        assert_eq!(finch.registers[1],0);
        assert_eq!(finch.flow_h,0);
    }

    #[test]
    fn h_search_8() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,Nop,Nop,HSearch,NopB,NopC,NopA,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop];
        finch.inst_h = 5;
        finch.increment();
        assert_eq!(finch.registers[2],0);
        assert_eq!(finch.registers[1],0);
        assert_eq!(finch.flow_h,0);
    }

    #[test]
    fn h_search_9() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,NopA,Nop,Nop,HSearch,NopB,NopC,NopA,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop];
        finch.inst_h = 6;
        finch.increment();
        assert_eq!(finch.registers[2],4);
        assert_eq!(finch.registers[1],0);
        assert_eq!(finch.flow_h,0);
    }

    #[test]
    fn h_search_10() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![Nop,Nop,Nop,NopB,NopC,NopA,NopA,Nop,Nop,HSearch,NopB,NopC,NopA,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop];
        finch.inst_h = 9;
        finch.increment();
        assert_eq!(finch.registers[2],4);
        assert_eq!(finch.registers[1],3);
        assert_eq!(finch.flow_h,3);
    }

    #[test]
    fn read_nop_label_1() {
        let mut finch: Finch = Finch::new(0,0,0);
        finch.memory = vec![NopB,NopC,NopA,HSearch,NopA,NopB,NopC,NopA
                            ,MovHead,
                            Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,Nop,NopA];
        finch.inst_h = 3;
        assert_eq!(read_nop_label(&finch.memory,finch.inst_h).len(),4);
    }

    // HCopy

    // If Label

    // HDivide


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
