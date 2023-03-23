use bkl::vm::{VM, Opcode};

fn main() {
    let mut vm = VM::new();
    
    // (5.5) / 2.2 * (1.0 - 2.0)
    vm.code.push((Opcode::Push, 5.5)); // pushes 5 into stack
    vm.code.push((Opcode::Push, 3.1)); // pushes 3 into stack
    vm.code.push((Opcode::Add, 0.0)); // summates 5 and 3
    vm.code.push((Opcode::Push, 2.2)); // pushes 2 into stack
    vm.code.push((Opcode::Div, 0.0)); // divides 8 by 2
    vm.code.push((Opcode::Push, 1.0)); // pushes 1 into stack
    vm.code.push((Opcode::Push, 2.0)); // pushes 2 into stack
    vm.code.push((Opcode::Sub, 0.0)); // substracts 2 from 1
    vm.code.push((Opcode::Mul, 0.0)); // multiplates 4 by -1
    
    vm.execute();
    let result = vm.pop();
    println!("Result: {}", result); // -3.9090909090909087
}
