pub enum Opcode {
    Push,
    Add,
    Sub,
    Mul,
    Div,
    Pop,
}

pub struct VM {
    pub code: Vec<(Opcode, f64)>,
    pub stack: Vec<f64>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            code: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }

    pub fn execute(&mut self) {
        for (opcode, operand) in &self.code {
            match opcode {
                Opcode::Push => self.stack.push(*operand),
                Opcode::Pop => drop(self.stack.pop()),
                Opcode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                Opcode::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                },
                Opcode::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                },
                Opcode::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                },
            }
        }
    }
}
