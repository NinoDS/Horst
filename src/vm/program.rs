use crate::vm::instruction::Instruction;
use crate::vm::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
}