use crate::vm::frame::CallFrame;
use crate::vm::instruction::Instruction;
use crate::vm::program::Program;
use crate::vm::value::{Function, Value};

mod value;
mod instruction;
mod program;
mod frame;

pub struct VM {
    frames: Vec<CallFrame>,
    stack: Vec<Value>,
    globals: Vec<Option<Value>>,
}

impl VM {
    pub fn new(program: Program, global_count: usize) -> Self {
        let mut globals: Vec<Option<Value>> = Vec::new();
        for _ in 0..global_count {
            globals.push(None);
        }

        Self{
            frames: vec![CallFrame{
                function: Function {
                    program,
                    arity: 0
                },
                ip: 0,
                base: 0
            }],
            stack: vec![],
            globals,
        }
    }

    pub fn run(&mut self) {
        macro_rules! binary_op {
            ($type:path, $op:tt) => {
                let b = self.pop();
                let a = self.pop();
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    self.push($type(a $op b));
                } else {
                    panic!();
                }
            };
        }
        
        while !self.frames.is_empty() && self.frame().ip < self.program().instructions.len() {
            let instruction = self.current_instruction();
            self.frame_mut().ip += 1;

            match instruction {
                Instruction::Add                        => { binary_op!(Value::Number, +); }
                Instruction::Subtract                   => { binary_op!(Value::Number, -); }
                Instruction::Multiply                   => { binary_op!(Value::Number, *); }
                Instruction::Divide                     => { binary_op!(Value::Number, /); }
                Instruction::Modulo                     => { binary_op!(Value::Number, %); }
                Instruction::Equal                      => { self.op_equal(); }
                Instruction::NotEqual                   => { self.op_not_equal(); }
                Instruction::Greater                    => { binary_op!(Value::Boolean, >); }
                Instruction::Less                       => { binary_op!(Value::Boolean, <); }
                Instruction::GreaterEqual               => { binary_op!(Value::Boolean, >=); }
                Instruction::LessEqual                  => { binary_op!(Value::Boolean, <=); }
                Instruction::Not                        => { self.op_not(); }
                Instruction::Negate                     => { self.op_negate(); }
                Instruction::SetLocal(index)      => { self.set_local(index); }
                Instruction::GetLocal(index)      => { self.get_local(index); }
                Instruction::DefineGlobal(index)  => { self.define_global(index); }
                Instruction::SetGlobal(index)     => { self.set_global(index); }
                Instruction::GetGlobal(index)     => { self.get_global(index); }
                Instruction::Return                     => { self.fn_return(); }
                Instruction::Call                       => { self.call(); }
                Instruction::Jump(pos)            => { self.frame_mut().ip = pos; }
                Instruction::JumpIfFalse(pos)     => { self.jump_if_false(pos) }
                Instruction::Constant(index)      => { self.push_constant(index); }
                Instruction::Pop                        => { self.pop(); }
                Instruction::Print                      => { println!("{}", self.pop()) }
            }
        }
    }

    fn jump_if_false(&mut self, pos: usize) {
        if self.pop().is_falsey() {
            self.frame_mut().ip = pos;
        }
    }

    fn get_global(&mut self, index: usize) {
        if self.globals[index].is_some() {
            self.push(self.globals[index].clone().unwrap());
        } else {
            panic!("Cannot get undefined variable");
        }
    }

    fn define_global(&mut self, index: usize) {
        self.globals[index] = Some(self.pop());
    }

    fn set_global(&mut self, index: usize) {
        if self.globals[index].is_some() {
            self.globals[index] = Some(self.pop());
        } else {
            panic!("Cannot set undefined variable");
        }
    }

    fn fn_return(&mut self) {
        let ret = self.pop();
        for _ in 0..self.frame().function.arity {
            self.pop();
        }
        self.push(ret);
        self.frames.pop();
    }

    fn call(&mut self) {
        if let Value::Function(f) = self.pop() {
            let arity = f.arity;
            let frame = CallFrame {
                function: f,
                ip: 0,
                base: self.stack.len() - arity,
            };
            self.frames.push(frame);
        } else {
            panic!("Cannot call value other than function!")
        }
    }

    fn set_local(&mut self, offset: usize) {
        let index = offset + self.frame().base;
        self.stack[index] = self.pop();
    }

    fn get_local(&mut self, offset: usize) {
        let index = offset + self.frame().base;
        self.push(self.stack[index].clone())
    }

    fn op_negate(&mut self) {
        if let Value::Number(n) = self.pop() {
            self.push(Value::Number(-n));
        } else {
            panic!("Cannot negate value other than number!")
        }
    }

    fn op_not(&mut self) {
        let value = Value::Boolean(self.pop().is_falsey());
        self.push(value);
    }

    fn op_equal(&mut self) {
        let value = Value::Boolean(self.pop() == self.pop());
        self.push(value);
    }

    fn op_not_equal(&mut self) {
        let value = Value::Boolean(self.pop() != self.pop());
        self.push(value);
    }

    fn push_constant(&mut self, index: usize) {
        let value = self.program().constants[index].clone();
        self.stack.push(value);
    }

    fn program(&self) -> &Program {
        &self.frame().function.program
    }

    fn current_instruction(&self) -> Instruction {
        self.program().instructions[self.frame().ip]
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        if let Some(value) = self.stack.pop() {
            value
        } else {
            panic!("Stack empty!")
        }
    }

    fn frame(&self) -> &CallFrame {
        if let Some(frame) = self.frames.last() {
            frame
        } else {
            panic!("Call stack empty!");
        }
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        if let Some(frame) = self.frames.last_mut() {
            frame
        } else {
            panic!("Call stack empty!");
        }
    }


}

#[cfg(test)]
mod tests {
    use crate::vm::instruction::Instruction;
    use crate::vm::program::Program;
    use crate::vm::value::{Function, Value};
    use crate::vm::VM;

    #[test]
    fn test_constant() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1)],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(2_f64))
    }

    #[test]
    fn test_pop() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Pop],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64))
    }

    #[test]
    fn test_print() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Print],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64))
    }

    #[test]
    fn test_add() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Add],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64 + 2_f64));
    }

    #[test]
    fn test_subtract() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Subtract],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64 - 2_f64));
    }

    #[test]
    fn test_multiply() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Multiply],
            constants: vec![Value::Number(3_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(3_f64 * 2_f64));
    }

    #[test]
    fn test_divide() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Divide],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64 / 2_f64));
    }

    #[test]
    fn test_modulo() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Modulo],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(1_f64 % 2_f64));
    }

    #[test]
    fn test_less_than() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Less],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 < 2_f64));
    }

    #[test]
    fn test_greater_than() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Greater],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 > 2_f64));
    }

    #[test]
    fn test_less_or_equal() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::LessEqual],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 <= 2_f64));
    }

    #[test]
    fn test_greater_or_equal() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::GreaterEqual],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 >= 2_f64));
    }

    #[test]
    fn test_equal() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Equal],
            constants: vec![Value::Number(1_f64), Value::Number(1_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 == 1_f64));
    }

    #[test]
    fn test_not_equal() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::NotEqual],
            constants: vec![Value::Number(1_f64), Value::Number(1_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(1_f64 != 1_f64));
    }

    #[test]
    fn test_not() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Not],
            constants: vec![Value::Boolean(false)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Boolean(true));
    }

    #[test]
    fn test_negate() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Negate],
            constants: vec![Value::Number(4.2)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(-4.2));
    }

    #[test]
    fn test_get_local() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::GetLocal(0)],
            constants: vec![Value::Number(4.2), Value::Null]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(4.2));
    }

    #[test]
    fn test_set_local() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Constant(2), Instruction::SetLocal(0)],
            constants: vec![Value::Number(4.2), Value::Null, Value::Boolean(false)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.stack[0], Value::Boolean(false));
    }

    #[test]
    fn test_define_global() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::DefineGlobal(0)],
            constants: vec![Value::Number(4.2)]
        };
        let mut vm = VM::new(program, 1);
        vm.run();
        assert_eq!(vm.globals[0], Some(Value::Number(4.2)));
    }

    #[test]
    fn test_set_global() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::DefineGlobal(0), Instruction::Constant(1), Instruction::SetGlobal(0)],
            constants: vec![Value::Number(4.2), Value::Boolean(true)],
        };
        let mut vm = VM::new(program, 1);
        vm.run();
        assert_eq!(vm.globals[0], Some(Value::Boolean(true)));
    }

    #[test]
    fn test_get_global() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::DefineGlobal(0), Instruction::Constant(1), Instruction::GetGlobal(0)],
            constants: vec![Value::Number(4.2), Value::Boolean(true)]
        };
        let mut vm = VM::new(program, 1);
        vm.run();
        assert_eq!(vm.globals[0], Some(Value::Number(4.2)));
    }

    #[test]
    fn test_return_global() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Return, Instruction::Constant(1)],
            constants: vec![Value::Number(4.2), Value::Boolean(true)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(4.2));
    }

    #[test]
    fn test_call() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Call],
            constants: vec![Value::Number(4.2), Value::Function( Function {
                program: Program {
                    instructions: vec![Instruction::GetLocal(0), Instruction::Constant(0), Instruction::Multiply],
                    constants: vec![Value::Number(2.0)]
                },
                arity: 1,
            })]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(8.4));
    }

    #[test]
    fn test_return() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::Call],
            constants: vec![Value::Number(4.2), Value::Function( Function {
                program: Program {
                    instructions: vec![Instruction::Constant(0), Instruction::Return, Instruction::GetLocal(0)],
                    constants: vec![Value::Number(2.0)]
                },
                arity: 1,
            })]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(2.0));
    }

    #[test]
    fn test_jump() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Jump(3), Instruction::Constant(1), Instruction::Constant(2), Instruction::Add],
            constants: vec![Value::Number(1_f64), Value::Number(2_f64), Value::Number(3_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(4_f64));
    }

    #[test]
    fn test_jump_if_false() {
        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::JumpIfFalse(4), Instruction::Constant(2), Instruction::Constant(3), Instruction::Add],
            constants: vec![Value::Number(1_f64), Value::Boolean(true), Value::Number(2_f64), Value::Number(3_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(5_f64));

        let program = Program {
            instructions: vec![Instruction::Constant(0), Instruction::Constant(1), Instruction::JumpIfFalse(4), Instruction::Constant(2), Instruction::Constant(3), Instruction::Add],
            constants: vec![Value::Number(1_f64), Value::Boolean(false), Value::Number(2_f64), Value::Number(3_f64)]
        };
        let mut vm = VM::new(program, 0);
        vm.run();
        assert_eq!(vm.pop(), Value::Number(4_f64));
    }
}
