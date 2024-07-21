mod operation;
mod generator;
mod simulator;
mod lexer;
mod utils;
mod token;
mod location;

fn main() {
    // check the command line arguments
    let (cl_args, src_file, asm_file, out_file, debug, run) = utils::command_line_args().unwrap_or_else(|e| panic!("[ERROR] {}:{}: {}", e.0, e.1, e.2));

    // lex a file to tokens
    let tokens = lexer::lex_file(&src_file); 
    if debug {
        tokens.iter().for_each(|t| println!("[INFO token]: {t}"));
    }

    // parse tokens to operations
    let operations = lexer::parse_tokens_to_operations(&tokens);
    if debug {
        operations.iter().for_each(|o| println!("[INFO op]: {o}"));
    }

    // simulate or compile the file
    let program = &operations;
    cl_args.iter().for_each(|arg| {
        match arg {
            utils::CLArgument::Simulation  => simulator::simulate_program(&program),
            utils::CLArgument::Compilation => {
                generator::create_assembly(&program, &asm_file).unwrap();

                utils::run_command(&["nasm", "-felf64", &asm_file]);
                utils::run_command(&["ld", asm_file.replace(".asm", ".o").as_str(), "-o", &out_file]);
                if run {
                    utils::run_command(&[ ["./", out_file.as_str()].concat().as_str() ]);
                }
            }
            _ => {},
        }
    });

}
