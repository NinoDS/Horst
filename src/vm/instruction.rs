#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    // Binary.
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Unary.
    Not,
    Negate,

    // Variables.
    SetLocal(usize),
    GetLocal(usize),
    DefineGlobal(usize),
    SetGlobal(usize),
    GetGlobal(usize),

    // Functions.
    Return,
    Call,

    // Control Flow.
    Jump(usize),
    JumpIfFalse(usize),

    // Other.
    Constant(usize),
    Pop,
    Print,
}