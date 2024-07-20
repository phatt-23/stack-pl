mod operation;
mod generator;
mod simulator;
mod lexer;
mod utils;
mod token;
mod location;

fn main() {
    // check the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if std::env::args().count() < 2 {
        utils::print_usage(&args);
        return;
    }

    let filepath = &args[2].as_str();
    let tokens = lexer::lex_file(filepath); // lex a file to tokens
    
    for t in &tokens {
        println!("[INFO token]: {t}");
    }

    // parse tokens to operations
    let operations = lexer::parse_tokens_to_operations(&tokens);
    
    for o in &operations {
        println!("[INFO op]: {o}");
    }

    



    // simulate or compile the file
    let program = &operations;
    match args[1].as_str() {
        "sim" => simulator::simulate_program(&program),
        "com" => {
            generator::create_assembly(&program, "output.asm").unwrap();
            generator::compile_assembly("program");
        }
        _ => utils::print_usage(&args),
    }
}
