use std::process::Command;

use crate::CLArgument;

pub fn print_usage() {
    let suffix = ".p";
    let args: Vec<String> = std::env::args().collect();
    println!("[ERROR]: {:?}", args);
    println!("[USAGE]: <COMPILER> <FLAGS> <ARGUMENT>");
    println!("     COMPILER: {}", args.get(0).unwrap());
    println!("        FILES: source files must end with {suffix:?} suffix");
    println!("        FLAGS: -s | --sim (default)              - interprets the code, doesnt create executable");
    println!("               -c | --com                        - generates assembly code, creates executable");
    println!("               -d | --dbg                        - prints out debug information");
    println!("               -r | --run                        - runs the generated executable after compilation");
    println!("               -a | --asm    <executable_name>   - specifies the path of generated assembly (intermediate code)");
    println!("               -o | --output <executable_name>   - specifies the path of generated executable");
}

pub fn run_command(args: &[&str]) {
    print_command_output(
        Command::new(args[0])
            .args(&args.to_vec()[1..])
            .output()
            .unwrap_or_else(|e| panic!("[ERROR]: {:?}: {e}", args[0])),
    );
}

fn print_command_output(output: std::process::Output) {
    if !&output.stdout.is_empty() {
        println!("[INFO $stdout]:");
        print!("{}", String::from_utf8_lossy(&output.stdout));
        println!("[INFO]: {}", output.status);
    }
    if !&output.stderr.is_empty() {
        println!("[INFO $stderr]:");
        print!("{}", String::from_utf8_lossy(&output.stderr));
        println!("[INFO]: {}", output.status);
    }
}
 
pub fn command_line_args() 
    -> Result< 
        (Vec<CLArgument>, String, String, String, bool), 
        (&'static str, u32, String) >
{
    let mut cl_args: Vec<CLArgument> = Vec::new();
    
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            s if s.ends_with(".p") => cl_args.push(CLArgument::File(arg.clone())),
            s if s.starts_with("--") => {
                match s {
                    "--dbg" => cl_args.push(CLArgument::Debug),
                    "--sim" => cl_args.push(CLArgument::Simulation),
                    "--com" => cl_args.push(CLArgument::Compilation),
                    "--run" => cl_args.push(CLArgument::RunCompiledProgram),
                    "--asm" => cl_args.push(CLArgument::Assembly(arg.clone())),
                    "--out" => cl_args.push(CLArgument::Output(arg.clone())),
                    e => {
                        print_usage();
                        return Err((file!(), line!(), format!("{:?} unknown full argument starting with '--'", e)));
                    }
                }
            }
            s if s.starts_with("-") => {
                let chars: Vec<char> = s[1..].chars().collect();
                for c in chars {
                    match c {
                        'd' => cl_args.push(CLArgument::Debug),
                        's' => cl_args.push(CLArgument::Simulation),
                        'c' => cl_args.push(CLArgument::Compilation),
                        'r' => cl_args.push(CLArgument::RunCompiledProgram),
                        'a' => cl_args.push(CLArgument::Assembly(arg.clone())),
                        'o' => cl_args.push(CLArgument::Output(arg.clone())),
                        e => {
                            print_usage();
                            return Err((file!(), line!(), format!("{:?} unknown short argument starting with '-'", e)));
                        }
                    }
                }
            }
            e => {
                print_usage();
                return Err((file!(), line!(), format!("{:?} unknown argument: not a source file nor argument", e)));
            }
        }
    }

    // if !cl_args.iter().any(|arg| matches!(arg, CLArgument::Compilation | CLArgument::Simulation)) {
    //     cl_args.push(CLArgument::Simulation)
    // }

    let src_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        if let CLArgument::File(file_name) = arg {
            Some(file_name.clone())
        } else {
            None
        }
    }).collect();

    if src_files.is_empty() {
        print_usage();
        return Err((file!(), line!(), format!("No a source file was provided")));
    }

    let asm_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        match arg {
            CLArgument::Assembly(file) => Some(file.clone()),
            _ => None
        }
    }).collect();

    let out_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        if let CLArgument::Output(file_name) = arg {
            Some(file_name.clone())
        } else {
            None
        }
    }).collect();

    let mut debug = false;
    if cl_args.contains(&CLArgument::Debug) {
        debug = true;
    }

    let asm_default = String::from("output.asm");
    let out_default = String::from("program");

    let src_file = src_files.first().unwrap(); 
    let asm_file = asm_files.first().unwrap_or(&asm_default);
    let out_file = out_files.first().unwrap_or(&out_default);

    Ok((cl_args, src_file.to_string(), asm_file.to_string(), out_file.to_string(), debug))
}