#[derive(Debug, PartialEq, Clone)]
pub enum Lexome {
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