mod operation;
mod generator;
mod lexer;
mod utils;
mod token;
mod location;
mod analyser;
mod keyword;
mod intrinsic;

fn main() -> Result<(), std::io::Error> {
    let cl_args = utils::process_command_line_args();
    let tokens = lexer::lex_file_to_tokens(cl_args.src_paths.first().unwrap())?; 
    if cl_args.dbg_flag {
        tokens.iter().for_each(|t| println!("[INFO]: {t}"));
    }
    let operations = analyser::compile_tokens_to_operations(tokens, &cl_args.inc_dirs, cl_args.expansion_limit)?;
    if cl_args.dbg_flag {
        operations.iter().for_each(|o| println!("[INFO]: {o}"));
    }
    generator::generate_linux_nasm_x86_64(&operations, cl_args.asm_path.clone().as_str())?;
    if cl_args.com_flag {
        utils::run_command(&["nasm", "-felf64", &cl_args.asm_path, "-o", &cl_args.obj_path]);
        utils::run_command(&["ld", &cl_args.obj_path, "-o", &cl_args.out_path]);
        if cl_args.run_flag {
            utils::run_command(&[format!("./{}", cl_args.out_path).as_str()]);
        }
    }
    Ok(())
}
