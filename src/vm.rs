pub enum Opcode {
    Push,
    Add,
    Sub,
    Mul,
    Div,
}

pub struct VM {
    code: Vec<(Opcode, i32)>,
    stack: Vec<i32>,
}

pub impl VM {
    pub fn new() -> VM {
        VM {
            code: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    pub fn execute(&mut self) {
        for (opcode, operand) in &self.code {
            match opcode {
                Opcode::Push => self.stack.push(*operand),
                Opcode::Add => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a + b);
                },
                Opcode::Sub => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a - b);
                },
                Opcode::Mul => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a * b);
                },
                Opcode::Div => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a / b);
                },
            }
        }
    }
}
