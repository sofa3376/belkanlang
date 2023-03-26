use std::str::FromStr;

use crate::vm::{VM, Opcode};

fn tokenize(expr: &str) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn parse(tokens: &mut Vec<String>) -> Result<Vec<String>, String> {
    if tokens.is_empty() {
        return Err("Unexpected EOF while reading.".to_string());
    }

    let token = tokens.remove(0);
    if token == "(" {
        let mut subexpr = Vec::new();
        while tokens.get(0) != Some(&")".to_string()) {
            subexpr.append(&mut parse(tokens)?);
        }
        tokens.remove(0); // Discard ")"
        Ok(subexpr)
    } else if token == ")" {
        Err("Unexpected )".to_string())
    } else {
        Ok(vec![token])
    }
}

fn generate_code(ast: &[String], code: &mut Vec<(Opcode, f64)>) -> Result<(), String> {
    if ast.is_empty() {
        return Ok(());
    }

    let op = &ast[0];
    match op.as_str() {
        "+" | "-" | "*" | "/" => {
            if ast.len() != 3 {
                return Err(format!("Expected 2 arguments for {}, got {}.", op, ast.len() - 1));
            }
            generate_code(&ast[1..], code)?;
            generate_code(&ast[2..], code)?;
            let opcode = match op.as_str() {
                "+" => Opcode::Add,
                "-" => Opcode::Sub,
                "*" => Opcode::Mul,
                "/" => Opcode::Div,
                _ => unreachable!(),
            };
            code.push((opcode, 0.0));
        }
        _ => {
            let number = f64::from_str(op).map_err(|_| format!("Invalid number: {}", op))?;
            code.push((Opcode::Push, number));
        }
    }

    Ok(())
}

pub fn eval(expr: &str, vm: &mut VM) -> Result<f64, String> {
    let tokens = tokenize(expr);
    let ast = parse(&mut tokens.clone())?;
    vm.code.clear();
    generate_code(&ast, &mut vm.code)?;
    vm.stack.clear();
    vm.execute();
    let result = vm.pop();
    Ok(result)
}
