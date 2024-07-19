use super::Token;
use super::operation::Operation;

pub fn simulate_program(program: Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();

    for op in program {
        match op.token {
            Token::Push => stack.push(op.value),
            Token::Dump => {
                let v = stack.pop();
                println!("{}", v.unwrap());
            }
            Token::Plus => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            Token::Eq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a == b) as i64);
            }
            Token::Le => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a < b) as i64);
            }
            Token::Gr => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push((a > b) as i64);
            }
            Token::End => {
                todo!();
            }
            Token::If => {
                todo!();
            }
            Token::Else => {
                todo!();
            }
            Token::Dup => {
                todo!();
            }
            Token::Do => todo!(),
            Token::While => todo!(),
            Token::EqGr => todo!(),
            Token::EqLe => todo!(),
            Token::Not => todo!(),
        }
    }
}
