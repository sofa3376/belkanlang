use std::io::{self, Write};

use bkl::vm::VM;
use bkl::interpreter::*;

fn main() {
    let mut vm = VM::new();
    
    println!("belkanLisp v0.0.1\nType your math expression or ^C to quit");
    
    loop {
        // infix: (2 * 3) + (8 / 2)
        // lisp (prefix): (+ (* 2 3) (/ 8 2))
        // rpn: 2 3 * 8 2 / +
        let mut expr = String::new();
        print!("--> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line");

        match eval(&expr, &mut vm) {
            Ok(result) => println!("Result: {}", result), // 10
            Err(err) => println!("Error: {}", err),
        }
    }
}
