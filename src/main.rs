use bkl::VM;

fn main() {
    let mut vm = VM::new();
    vm.code.push((Opcode::Push, 5));
    vm.code.push((Opcode::Push, 3));
    vm.code.push((Opcode::Add, 0));
    vm.execute();
    let result = vm.pop();
    println!("Result: {}", result);
}
