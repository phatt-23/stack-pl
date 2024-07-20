mod operation;
mod compiler;
mod simulator;
mod lexer;
mod utils;

fn main() {
    // check the command line arguments
    let args: Vec<String> = std::env::args().collect();
    if std::env::args().count() < 2 {
        utils::print_usage(&args);
        return;
    }

    // open the file and compose the program from given file
    let input_filepath = &args[2].as_str();
    let mut program: Vec<operation::Operation> = Vec::new(); 
    lexer::lex_file(input_filepath, &mut program);
    for op in &program {
        println!("[INFO]: {:?}", op);
    }

    // simulate or compile the file
    match args[1].as_str() {
        "sim" => simulator::simulate_program(&program),
        "com" => {
            compiler::create_assembly(&program, "output.asm").unwrap();
            compiler::compile_assembly("program");
        }
        _ => utils::print_usage(&args),
    }
}
