mod operation;
mod generator;
mod simulator;
mod lexer;
mod utils;
mod token;
mod location;
mod analyser;

fn main() -> Result<(), std::io::Error> {
    let cl_args = utils::process_command_line_args().unwrap_or_else(|e| panic!("[ERROR] {} {}", e.0, e.1));

    let tokens = lexer::lex_file_to_tokens(cl_args.src_files.first().unwrap())?; 
    if cl_args.dbg_flag {
        tokens.iter().for_each(|t| println!("[INFO token]: {t}"));
    }

    let operations = analyser::compile_tokens_to_operations(tokens)?;
    if cl_args.dbg_flag {
        operations.iter().for_each(|o| println!("[INFO op]: {o}"));
    }
    
    generator::generate_linux_nasm_x86_64(&operations, cl_args.asm_file.clone().as_str())?;
    
    if cl_args.sim_flag {
        simulator::simulate_program(operations);
    }

    if cl_args.com_flag {
        utils::run_command(&["nasm", "-felf64", &cl_args.asm_file, "-o", &cl_args.obj_file]);
        utils::run_command(&["ld", &cl_args.obj_file, "-o", &cl_args.out_file]);
        if cl_args.run_flag {
            utils::run_command(&[format!("./{}", cl_args.out_file).as_str()]);
        }
    }

    Ok(())
}
