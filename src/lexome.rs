#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Lexome {
    Nop,
    NopA,
    NopB,
    NopC,
    IfNEqu,
    IfLess,
    Pop,
    Push,
    SwapStk,
    Swap,
    ShiftR,
    ShiftL,
    Inc,
    Dec,
    Add,
    Sub,
    Nand,
    IO,
    HAlloc,
    HDivide,
    HCopy,
    HSearch,
    MovHead,
    JmpHead,
    GetHead,
    IfLabel,
    SetFlow,
}

impl Lexome {
    // TODO Lexome reading from files
}

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