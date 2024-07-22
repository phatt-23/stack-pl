mod operation;
mod generator;
mod simulator;
mod lexer;
mod utils;
mod token;
mod location;
mod analyser;

fn main() -> Result<(), std::io::Error> {
    let (src_file, asm_file, out_file, sim_mode, com_mode, debug, run) = utils::command_line_args().unwrap_or_else(|e| panic!("[ERROR] {}:{}: {}", e.0, e.1, e.2));

    let tokens = lexer::lex_file_to_tokens(&src_file)?; 
    if debug {
        tokens.iter().for_each(|t| println!("[INFO token]: {t}"));
    }

    let operations = analyser::compile_tokens_to_operations(&tokens);
    if debug {
        operations.iter().for_each(|o| println!("[INFO op]: {o}"));
    }
    
    generator::generate_linux_nasm_x86_64(&operations, &asm_file).unwrap();
    
    if sim_mode {
        simulator::simulate_program(&operations);
    }

    if com_mode {
        utils::run_command(&["nasm", "-felf64", &asm_file]);
        utils::run_command(&["ld", asm_file.replace(".asm", ".o").as_str(), "-o", &out_file]);
        if run {
            utils::run_command(&[ ["./", out_file.as_str()].concat().as_str() ]);
        }
    }

    Ok(())
}
