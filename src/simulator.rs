use super::Token;
use super::operation::Operation;

pub fn simulate_program(program: Vec<Operation>) {
    println!("[INFO]: Simulating the program");
    let mut stack: Vec<i64> = Vec::new();

    let mut ip: usize = 0;
    while ip < program.len() {
        let op = program[ip];
        match op.token {
            Token::Push => {
                stack.push(op.value);
            }
            Token::Dump => {
                println!("{}", stack.pop().expect("[ERROR]: (Empty Stack) <dump> 'dump' expects 1 operand"));
            }
            Token::Plus => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <plus> '+' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <plus> '+' expects 2 operands (second operand)");
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <minus> '-' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <minus> '-' expects 2 operands (second operand)");
                stack.push(a - b);
            }
            Token::Eq => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (second operand)");
                stack.push((a == b) as i64);
            }
            Token::NotEq => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal> '=' expects 2 operands (second operand)");
                stack.push((a != b) as i64);
            }
            Token::Le => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <less> '<' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <less> '<' expects 2 operands (second operand)");
                stack.push((a < b) as i64);
            }
            Token::Gr => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <greater> '>' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <greater> '>' expects 2 operands (second operand)");
                stack.push((a > b) as i64);
            }
            Token::End => {
                if op.value > 0 {
                    ip = (op.value - 1) as usize;
                }
            }
            Token::If => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <if-statement> 'if' expects 1 operand") != 0;
                if a == false {
                    ip = (op.value + 1) as usize;
                }
            }
            Token::Else => {
                ip = op.value as usize;
            }
            Token::Dup => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <duplicate> 'dup' expects 1 operand");
                stack.push(a);
                stack.push(a);
            }
            Token::Do => {
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <do-statement> 'do' expects 1 operand") != 0;
                if a == false {
                    ip = (op.value) as usize;
                }
            }
            Token::While => {
                // nothing
            }
            Token::EqGr => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal-greater> '>=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal-greater> '>=' expects 2 operands (second operand)");
                stack.push((a >= b) as i64);
            }
            Token::EqLe => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <equal-less> '<=' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <equal-less> '<=' expects 2 operands (second operand)");
                stack.push((a <= b) as i64);
            }
            Token::Not => {
                let a = stack.pop().unwrap() != 0;
                stack.push((!a) as i64);
            }
            Token::Multiply => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <multiply> '*' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <multiply> '*' expects 2 operands (second operand)");
                stack.push((a * b) as i64);
            }
            Token::Divide => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <divide> '/' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <divide> '/' expects 2 operands (second operand)");
                stack.push((a / b) as i64);
            }
            Token::Modulo => {
                let b = stack.pop().expect("[ERROR]: (Empty Stack) <modulo> '%' expects 2 operands (first operand)");
                let a = stack.pop().expect("[ERROR]: (Empty Stack) <modulo> '%' expects 2 operands (second operand)");
                stack.push((a % b) as i64);
            }
        }
        ip += 1;
    }
}
