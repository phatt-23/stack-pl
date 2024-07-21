use std::process::Command;

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
    println!("               -r | --run                        - runs the generated executable after compilation (must be used with -c or --com)");
    println!("               -a | --asm    <executable_name>   - specifies the path of generated assembly (intermediate code)");
    println!("               -o | --output <executable_name>   - specifies the path of generated executable");
}

pub fn run_command(args: &[&str]) {
    // println!("{args:?}");
    print_command_output(
        Command::new(args[0])
            .args(&args.to_vec()[1..])
            .output()
            .unwrap_or_else(|e| panic!("[ERROR]: {:?}: {e}", args[0])),
    );
}

fn print_command_output(output: std::process::Output) {
    if !output.stdout.is_empty() {
        println!("[INFO $stdout]:");
        match std::str::from_utf8(&output.stdout) {
            Ok(stdout) => print!("{}", stdout),
            Err(_) => print!("{:?}", &output.stdout),
        }
        println!("[INFO]: {}", output.status);
    }
    if !output.stderr.is_empty() {
        println!("[INFO $stderr]:");
        match std::str::from_utf8(&output.stderr) {
            Ok(stderr) => print!("{}", stderr),
            Err(_) => print!("{:?}", &output.stderr),
        }
        println!("[INFO]: {}", output.status);
    }
}


#[derive(Eq, PartialEq)]
pub enum CLArgument {
    Debug,
    Simulation,
    Compilation,
    RunCompiledProgram,
    File(String),
    Output(String),
    Assembly(String),
}

fn handle_flag_with_file_arg(flag: &str, file: &'static str, line: u32, arg: Option<&String>) -> Result<String, (&'static str, u32, String)>  {
    let file_arg = arg.unwrap_or_else(|| panic!("[ERROR]: {file}:{line}: ({flag}) no filepath provided"));
    if file_arg.starts_with("-") {
        Err((file, line, format!("[ERROR]: ({flag}) invalid filepath: {file_arg}")))
    } else {
        Ok(file_arg.to_string())
    }
}

pub fn command_line_args() 
    -> Result< 
        (Vec<CLArgument>, String, String, String, bool, bool), 
        (&'static str, u32, String) >
{
    let mut cl_args: Vec<CLArgument> = Vec::new();
    
    let args: Vec<String> = std::env::args().collect();
    let mut index: usize = 1;
    while index < args.len() {
        let arg = &args[index];

        match arg.as_str() {
            s if s.ends_with(".p") => cl_args.push(CLArgument::File(arg.clone())),
            s if s.starts_with("--") => {
                match s {
                    "--dbg" => cl_args.push(CLArgument::Debug),
                    "--sim" => cl_args.push(CLArgument::Simulation),
                    "--com" => cl_args.push(CLArgument::Compilation),
                    "--run" => cl_args.push(CLArgument::RunCompiledProgram),
                    "--asm" => {
                        let file = handle_flag_with_file_arg("--asm", file!(), line!(), args.get(index + 1))?;
                        cl_args.push(CLArgument::Assembly(file));
                        index += 1;
                    }
                    "--out" => {
                        let file = handle_flag_with_file_arg("--out", file!(), line!(), args.get(index + 1))?;
                        cl_args.push(CLArgument::Output(file.clone()));
                        index += 1;
                    }
                    e => {
                        print_usage();
                        return Err((file!(), line!(), format!("{:?} unknown full argument starting with '--'", e)));
                    }
                }
            }
            // TODO: these must be handled on their own, cant be a part of multiple flags, they take in parameters
            "-a" => {
                let file = handle_flag_with_file_arg("-a", file!(), line!(), args.get(index + 1))?;
                cl_args.push(CLArgument::Assembly(file.clone()));
                index += 1;
            }
            "-o" => {
                let file = handle_flag_with_file_arg("-o", file!(), line!(), args.get(index + 1))?;
                println!("{file}");
                cl_args.push(CLArgument::Output(file.clone()));
                index += 1;
            }
            s if s.starts_with("-") => {
                let chars: Vec<char> = s[1..].chars().collect();
                for c in chars {
                    match c {
                        'd' => cl_args.push(CLArgument::Debug),
                        's' => cl_args.push(CLArgument::Simulation),
                        'c' => cl_args.push(CLArgument::Compilation),
                        'r' => cl_args.push(CLArgument::RunCompiledProgram),
                        'a' => return Err((file!(), line!(), format!("short flag 'a' must be used independently"))),
                        'o' => return Err((file!(), line!(), format!("short flag 'o' must be used independently"))),
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

        index += 1;
    }

    
    let sim_mode = if cl_args.iter().any(|arg| matches!(arg, CLArgument::Simulation)) {true} else {false};
    let com_mode = if cl_args.iter().any(|arg| matches!(arg, CLArgument::Compilation)) {true} else {false};
    if !(com_mode || sim_mode) {
        return Err((file!(), line!(), format!("provide (-s, --sim) for simulation or (-c, --com) for compilation")));
    }
    
    let src_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        if let CLArgument::File(file_name) = arg { Some(file_name.clone()) } else { None }
    }).collect();

    let src_file = src_files.first().ok_or_else(|| ((file!(), line!(), format!("No a source file was provided"))) )?; 

    let asm_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        match arg {
            CLArgument::Assembly(file) => Some(file.clone()),
            _ => None
        }
    }).collect();

    let out_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        if let CLArgument::Output(file_name) = arg { Some(file_name.clone()) } else { None }
    }).collect();

    let run = if cl_args.contains(&CLArgument::RunCompiledProgram) {
        if !com_mode {
            return Err((file!(), line!(), format!("(-r, --run) flags must be ran with compilation mode (-c, --com)")));
        }
        true
    } else {false};

    let debug = if cl_args.contains(&CLArgument::Debug) {true} else {false};

    let asm_file = asm_files.first().map_or_else(|| "output.asm".to_string(), |s| s.to_string());
    let out_file = out_files.first().map_or_else(|| "program".to_string(),    |s| s.to_string());

    Ok((cl_args, src_file.to_string(), asm_file.to_string(), out_file.to_string(), debug, run))
}

