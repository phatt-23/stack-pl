use std::{fs, io::Write, process::Command};

const LANGUAGE_SUFFIX: &'static str = ".p";

pub fn print_usage() {
    let args: Vec<String> = std::env::args().collect();
    println!("[ERROR]: Invalid command: {}", args.join(" "));
    println!("[USAGE]: <COMPILER> <FLAGS> <ARGUMENT>");
    println!("     COMPILER: {}", args.get(0).unwrap());
    println!("        FILES: source files must end with {:?} suffix", LANGUAGE_SUFFIX);
    println!("        FLAGS: -s | --sim                        - interprets the code, doesnt create executable");
    println!("               -c | --com                        - generates assembly code, creates executable");
    println!("               -d | --dbg                        - prints out debug information");
    println!("               -r | --run                        - runs the generated executable after compilation (must be used with -c or --com)");
    println!("               -a | --asm    <path>              - specifies the path of generated assembly (intermediate code)");
    println!("               -o | --output <path>              - specifies the path of generated executable");
}

pub fn run_command(args: &[&str]) {
    let output = Command::new(args[0])
        .args(&args.to_vec()[1..])
        .output()
        .unwrap_or_else(|e| panic!("[ERROR]: {:?}: {e}", args[0]));
    
    if !output.stdout.is_empty() {
        println!("[INFO $stdout]: The stdout of command: {}", args.join(" "));
        match std::str::from_utf8(&output.stdout) {
            Ok(stdout) => print!("{}", stdout),
            Err(_) => print!("{:?}", &output.stdout),
        }
        println!("[INFO]: {}", output.status);
    }

    if !output.stderr.is_empty() {
        println!("[INFO $stderr]: The stderr of command: {}", args.join(" "));
        match std::str::from_utf8(&output.stderr) {
            Ok(stderr) => print!("{}", stderr),
            Err(_) => print!("{:?}", &output.stderr),
        }
        println!("[INFO]: {}", output.status);
    }
}

#[derive(Eq, PartialEq)]
enum CommandLineArgKind {
    Debug,
    Simulation,
    Compilation,
    RunCompiledProgram,
    File(String),
    Output(String),
    Assembly(String),
}

pub struct CommandLineArgs {
    pub src_files:  Vec<String>,
    pub asm_file:   String,
    pub obj_file:   String,
    pub out_file:   String,
    pub sim_flag:   bool,
    pub com_flag:   bool,
    pub dbg_flag:   bool,
    pub run_flag:   bool,
}

impl CommandLineArgs {
    
    const TARGET_DIR:   &'static str = "tp_target";
    const DEBUG_DIR:    &'static str = "debug";
    const OBJECT_DIR:   &'static str = "obj";
    const ASM_FILENAME: &'static str = "program.asm";
    const OBJ_FILENAME: &'static str = "program.o";
    const OUT_FILENAME: &'static str = "program.out";

    pub fn new(
        src_files: Vec<&String>,
        asm_file: Option<&String>,
        out_file: Option<&String>,
        obj_file: Option<&String>,
        sim_flag: bool,
        com_flag: bool,
        dbg_flag: bool,
        run_flag: bool,
    ) -> Self {
        let default_asm_path: &String = &format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::ASM_FILENAME);
        let default_obj_path: &String = &format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::OBJ_FILENAME);
        let default_out_path: &String = &format!("{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OUT_FILENAME);

        Self {
            src_files: src_files.iter().map(|f| f.to_string()).collect(),
            asm_file: asm_file.unwrap_or(default_asm_path).clone(),
            out_file: out_file.unwrap_or(default_out_path).clone(),
            obj_file: obj_file.unwrap_or(default_obj_path).clone(),
            sim_flag,
            com_flag,
            dbg_flag,
            run_flag,
        }
    }
}

impl Default for CommandLineArgs {
    fn default() -> Self {
        Self {
            src_files:  vec![],
            asm_file:   format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::ASM_FILENAME),
            obj_file:   format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::OBJ_FILENAME),
            out_file:   format!("{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OUT_FILENAME),
            sim_flag:   false,
            com_flag:   false,
            dbg_flag:   false,
            run_flag:   false, 
        }
    }
}

type ErrorMessageAndLocation = (&'static str, u32, String);

fn handle_flag_with_file_arg(flag: &str, file: &'static str, line: u32, arg: Option<&String>) -> Result<String, ErrorMessageAndLocation>  {
    let file_arg = arg.unwrap_or_else(|| panic!("[ERROR]: {file}:{line}: ({flag}) no filepath provided"));
    if file_arg.starts_with("-") {
        Err((file, line, format!("[ERROR]: ({flag}) invalid filepath: {file_arg}")))
    } else {
        Ok(file_arg.to_string())
    }
}

pub fn process_command_line_args() -> Result<CommandLineArgs, ErrorMessageAndLocation> {
    let args: Vec<String> = std::env::args().collect();
    let mut cl_args: Vec<CommandLineArgKind> = Vec::new();
    let mut index: usize = 1;
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            s if s.ends_with(LANGUAGE_SUFFIX) => cl_args.push(CommandLineArgKind::File(arg.clone())),
            s if s.starts_with("--") => {
                match s {
                    "--dbg" => cl_args.push(CommandLineArgKind::Debug),
                    "--sim" => cl_args.push(CommandLineArgKind::Simulation),
                    "--com" => cl_args.push(CommandLineArgKind::Compilation),
                    "--run" => cl_args.push(CommandLineArgKind::RunCompiledProgram),
                    "--asm" => {
                        let file = handle_flag_with_file_arg("--asm", file!(), line!(), args.get(index + 1))?;
                        cl_args.push(CommandLineArgKind::Assembly(file));
                        index += 1;
                    }
                    "--out" => {
                        let file = handle_flag_with_file_arg("--out", file!(), line!(), args.get(index + 1))?;
                        cl_args.push(CommandLineArgKind::Output(file.clone()));
                        index += 1;
                    }
                    e => {
                        print_usage();
                        return Err((file!(), line!(), format!("{:?} unknown full argument starting with '--'", e)));
                    }
                }
            }
            // these must be handled on their own, cant be a part of multiple flags, they take in parameters
            "-a" => {
                let file = handle_flag_with_file_arg("-a", file!(), line!(), args.get(index + 1))?;
                cl_args.push(CommandLineArgKind::Assembly(file.clone()));
                index += 1;
            }
            "-o" => {
                let file = handle_flag_with_file_arg("-o", file!(), line!(), args.get(index + 1))?;
                cl_args.push(CommandLineArgKind::Output(file.clone()));
                index += 1;
            }
            // these flags can be merged in one, can be part of multiple flags, dont take any parameters
            s if s.starts_with("-") => {
                let chars: Vec<char> = s[1..].chars().collect();
                for c in chars {
                    match c {
                        'd' => cl_args.push(CommandLineArgKind::Debug),
                        's' => cl_args.push(CommandLineArgKind::Simulation),
                        'c' => cl_args.push(CommandLineArgKind::Compilation),
                        'r' => cl_args.push(CommandLineArgKind::RunCompiledProgram),
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
                return Err((file!(), line!(), format!("Unknown compiler argument {:?}, not a source file nor argument!", e)));
            }
        }

        index += 1;
    }

    let com_flag = if cl_args.iter().any(|arg| matches!(arg, CommandLineArgKind::Compilation)) {true} else {false};
    let sim_flag = if cl_args.iter().any(|arg| matches!(arg, CommandLineArgKind::Simulation)) {true} else {false};
    // if !(com_flag || sim_flag) { Err((file!(), line!(), format!("provide (-s, --sim) for simulation or (-c, --com) for compilation"))) }
    
    let src_files: Vec<&String> = cl_args.iter().filter_map(|arg| { if let CommandLineArgKind::File(file_name) = arg { Some(file_name) } else { None } }).collect();

    
    if src_files.len() < 1 {
        return Err((file!(), line!(), format!("No a source file was provided")));
    }


    let asm_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        match arg {
            CommandLineArgKind::Assembly(file) => Some(file.clone()),
            _ => None
        }
    }).collect();
    
    let out_files: Vec<String> = cl_args.iter().filter_map(|arg| {
        if let CommandLineArgKind::Output(file_name) = arg { Some(file_name.clone()) } else { None }
    }).collect();
    
    let run_com_flag = if cl_args.contains(&CommandLineArgKind::RunCompiledProgram) {
        if !com_flag {
            return Err((file!(), line!(), format!("(-r, --run) flags must be ran with compilation mode (-c, --com)")));
        }
        true
    } else {false};

    let debug_flag = if cl_args.contains(&CommandLineArgKind::Debug) {true} else {false};

    let asm_file: Option<&String> = asm_files.first();
    let out_file: Option<&String> = out_files.first();
    let obj_file: Option<String> = if let Some(f) = &asm_file { 
        if f.ends_with(".asm") { 
            Some( f.replace(".asm", ".o") )
        } else { panic!("[ERROR]: Specified ASM file output must have '.asm' suffix"); }
    } else {None};
    let obj_file: Option<&String> = obj_file.as_ref();

    
    let cl_args = CommandLineArgs::new(
        src_files,
        asm_file,
        out_file,
        obj_file,
        sim_flag,
        com_flag,
        debug_flag,
        run_com_flag,
    );
    
    use std::path::{PathBuf, Path};
    
    let callers_dir = std::env::current_dir().unwrap();
    // println!("[INFO]: You are in directory: {:?}", callers_dir.display());

    let path_buf = PathBuf::from(&cl_args.asm_file);
    let asm_obj_dir = path_buf.parent().unwrap(); 

    let path_buf = PathBuf::from(&cl_args.out_file);
    let out_dir = path_buf.parent().unwrap();

    // println!("[INFO]: ASM Obj Directory: {:?}", asm_obj_dir);
    // println!("[INFO]: Out Directory: {:?}", out_dir);

    // for (i, e) in std::fs::read_dir(callers_dir).unwrap().enumerate() {
    //     let e = e.unwrap();
    //     println!("[INFO]: [{i:>3}]: {:<10}", &e.path().display());
    // }
    
    let mut dirs_to_check: Vec<_> = Vec::new();
    let mut ancestors = out_dir.ancestors();
    while let Some(p) = ancestors.next() {
        // println!("[INFO]: Ancestors: {:?}", p);
        dirs_to_check.push(p);    
    }

    let mut ancestors = asm_obj_dir.ancestors();
    while let Some(p) = ancestors.next() {
        // println!("[INFO]: Ancestors: {:?}", p);
        dirs_to_check.push(p);    
    }

    dirs_to_check.sort();
    dirs_to_check.dedup();

    // println!("[INFO]: Directories to check for whether they exist: {:?}", dirs_to_check);
    for d in dirs_to_check {
        let path = format!("{}/{}", &callers_dir.display(), &d.display());
        if !Path::new(&path).exists() {
            loop {
                println!("[WARN]: This path does not exist: {:?}", path);
                print!("[PROMPT]: Do you wish to create it? Please enter (y) (n) (q): ");
                let _ = std::io::stdout().flush();

                let mut input_buffer = String::new();
                std::io::stdin().read_line(&mut input_buffer).expect("[ERROR]: Input read incorrectly");

                if let Some('\n') = input_buffer.chars().next_back() { input_buffer.pop(); } // remove new line escape char
                if let Some('\r') = input_buffer.chars().next_back() { input_buffer.pop(); } // for windows ig 
                match input_buffer.as_str() {
                    "y" | "Y" | "yes" | "Yes" | "YES" => {
                        fs::create_dir_all(&path).unwrap_or_else(|e| panic!("[ERROR]: Failed to create the path {:?}, message: {e}", path));
                        println!("[INFO]: Succesfully created path: {path:?}");
                        break;
                    }
                    "n" | "N" | "no" | "No" | "NO" => break,
                    "q" | "Q" | "quit" | "Quit" | "QUIT" => std::process::exit(0),
                    _ => println!("[ERROR]: Entered {:?} is not a valid option", input_buffer)
                }
            }
        }
    }

    Ok(cl_args)
}

