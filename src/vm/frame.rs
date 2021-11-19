use crate::vm::value::Function;

pub(crate) struct CallFrame {
    pub function: Function,
    pub ip: usize,
    pub base: usize,
}