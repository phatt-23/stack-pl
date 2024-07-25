use std::{fs, io::Write, process::Command};

const LANGUAGE_SUFFIX: &str = ".p";

pub fn print_usage() {
    println!("[USAGE]: <compiler> [options]", );
    println!("     compiler:");
    println!("         {}", std::env::current_exe().unwrap().display());
    println!("     files:");
    println!("         Source files must end with {:?} suffix", LANGUAGE_SUFFIX);
    println!("     options:");
    println!("         -c, --com             Generates assembly code, creates executable");
    println!("         -d, --dbg             Prints out debug information");
    println!("         -r, --run             Runs the generated executable after compilation (must be used with -c or --com)");
    println!("         -a, --asm    <path>   Specifies the path of generated assembly (intermediate code)");
    println!("         -o, --output <path>   Specifies the path of generated executable");
    println!("         -h, --help            Print usage");
    println!("         -I, --include         Include path the compiler will search for to include");
}

pub fn run_command(args: &[&str]) {
    // dbg!(&args);
    let output = Command::new(args[0])
        .args(&args.to_vec()[1..])
        .output()
        .unwrap_or_else(|e| panic!("[ERROR]: {:?}: {e}", args[0]));
    
    if !output.stdout.is_empty() {
        println!("[INFO]: The stdout of command: {}", args.join(" "));
        match std::str::from_utf8(&output.stdout) {
            Ok(stdout) => print!("{}", stdout),
            Err(_) => print!("{:?}", &output.stdout),
        }
        println!("[INFO]: {}", output.status);
    }

    if !output.stderr.is_empty() {
        println!("[INFO]: The stderr of command: {}", args.join(" "));
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
    Compilation,
    RunCompiledProgram,
    File(String),
    Output(String),
    Assembly(String),
    IncludeDirectory(String),
}

#[derive(Debug)]
pub struct CommandLineArgs {
    pub src_files:  Vec<String>,
    pub asm_file:   String,
    pub obj_file:   String,
    pub out_file:   String,
    pub com_flag:   bool,
    pub dbg_flag:   bool,
    pub run_flag:   bool,
    pub inc_dirs:   Vec<String>,
}

impl CommandLineArgs {
    
    const TARGET_DIR:   &'static str = "toi_target";
    const DEBUG_DIR:    &'static str = "debug";
    const OBJECT_DIR:   &'static str = "obj";
    const ASM_FILENAME: &'static str = "program.asm";
    const OBJ_FILENAME: &'static str = "program.o";
    const OUT_FILENAME: &'static str = "program.out";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        src_files: Vec<&String>,
        asm_file: Option<&String>,
        out_file: Option<&String>,
        obj_file: Option<&String>,
        com_flag: bool,
        dbg_flag: bool,
        run_flag: bool,
        inc_dirs: Vec<&String>,
    ) -> Self {
        let default_asm_path: &String = &format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::ASM_FILENAME);
        let default_obj_path: &String = &format!("{}/{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OBJECT_DIR, Self::OBJ_FILENAME);
        let default_out_path: &String = &format!("{}/{}/{}", Self::TARGET_DIR, Self::DEBUG_DIR, Self::OUT_FILENAME);

        Self {
            src_files: src_files.iter().map(|f| f.to_string()).collect(),
            asm_file: asm_file.unwrap_or(default_asm_path).clone(),
            out_file: out_file.unwrap_or(default_out_path).clone(),
            obj_file: obj_file.unwrap_or(default_obj_path).clone(),
            com_flag,
            dbg_flag,
            run_flag,
            inc_dirs: inc_dirs.iter().map(|d| d.to_string()).collect(),
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
            com_flag:   false,
            dbg_flag:   false,
            run_flag:   false, 
            inc_dirs:   vec![],
        }
    }
}

fn handle_file<'a>(index: usize, args: &'a Vec<String>, arg: &'a String) -> &'a String {
    args.get(index + 1).unwrap_or_else(|| {
        println!("[ERROR]: {:?} No filepath provided!", arg);
        std::process::exit(1);
    })
}

pub fn process_command_line_args() -> CommandLineArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut cl_args: Vec<CommandLineArgKind> = Vec::new();
    let mut index: usize = 1;
    while index < args.len() {
        let arg = &args[index];
        // dbg!(arg);
        match arg.as_str() {
            // source files
            s if s.ends_with(LANGUAGE_SUFFIX) => cl_args.push(CommandLineArgKind::File(arg.clone())),
            
            "--dbg" | "-d" => cl_args.push(CommandLineArgKind::Debug),
            "--com" | "-c" => cl_args.push(CommandLineArgKind::Compilation),
            "--run" | "-r" => cl_args.push(CommandLineArgKind::RunCompiledProgram),
            "--asm" | "-a" => {
                cl_args.push(CommandLineArgKind::Assembly(handle_file(index, &args, arg).clone()));
                index += 1;
            }
            "--out" | "-o" => {
                cl_args.push(CommandLineArgKind::Output(handle_file(index, &args, arg).clone()));
                index += 1;
            }
            "--include" | "-I" => {
                let path = args.get(index + 1).unwrap_or_else(|| {
                    println!("[ERROR]: {:?} No include directory provided!", arg);
                    std::process::exit(1);
                });
                if !fs::metadata(path).unwrap_or_else(|e| {
                    println!("[ERROR]: {:?} {path} Path provided doesn't exist: {e}", arg);
                    std::process::exit(1);
                }).is_dir() {
                    println!("[ERROR]: {:?} {path}: Path provided not a directory!", arg);
                    std::process::exit(1);
                }
                cl_args.push(CommandLineArgKind::IncludeDirectory(path.clone()));
                index += 1;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            s if s.starts_with('-') => {
                assert!(s.len() >= 2, "[ERROR]: Found dash with no letter as compiler argument!");
                // trim off the dash
                let chars: &str = &s[1..]; 
                let flag = &chars[0..1];
                let argument = &chars[1..];
                // First check the if they fall into those that must be checked on their own 
                match flag {
                    "a" => cl_args.push(CommandLineArgKind::Assembly(argument.to_string())),
                    "o" => cl_args.push(CommandLineArgKind::Output(argument.to_string())),
                    "I" => {
                        if !fs::metadata(argument).unwrap_or_else(|e| {
                            println!("[ERROR]: {:?} {argument}: Path provided doesn't exist: {e}", arg);
                            std::process::exit(1);
                        }).is_dir() {
                            println!("[ERROR]: {:?} {argument}: Path provided not a directory!", arg);
                            std::process::exit(1);
                        }
                        cl_args.push(CommandLineArgKind::IncludeDirectory(argument.to_string()));
                    }
                    _ => {
                        let mut i: usize = 0;
                        while let Some(letter) = chars.get(i..=i) {
                            match letter {
                                "a" | "o"| "I" => {
                                    println!("[ERROR]: {:?} Flag {:?} in merged single dash flags must be used independently!", chars, letter);
                                    std::process::exit(1);
                                }
                                // These flags can be merged in one, can be part of multiple flags, they dont take any parameters
                                "d" => cl_args.push(CommandLineArgKind::Debug),
                                "c" => cl_args.push(CommandLineArgKind::Compilation),
                                "r" => cl_args.push(CommandLineArgKind::RunCompiledProgram),
                                unknown => {
                                    println!("[ERROR]: Invalid command: {}", args.join(" "));
                                    println!("[ERROR]: {:?} In merged short flags, {:?} is an unknown flag, run with --help for usage!", chars, unknown);
                                    std::process::exit(1);
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
            e => {
                println!("[ERROR]: {:?} Unknown compiler argument, not a source file nor an argument!", e);
                std::process::exit(1);
            }
        }

        index += 1;
    }

    let com_flag = cl_args.iter().any(|arg| matches!(arg, CommandLineArgKind::Compilation)); 
    
    let src_files: Vec<&String> = cl_args.iter().filter_map(|arg| { if let CommandLineArgKind::File(file_name) = arg { Some(file_name) } else { None } }).collect();
    if src_files.is_empty() {
        println!("[ERROR]: No source file was provided.");
        std::process::exit(1);
    }

    let inc_dirs: Vec<_> = cl_args.iter().filter_map(|arg| { if let CommandLineArgKind::IncludeDirectory(inc_dir) = arg {Some(inc_dir)} else {None} }).collect();

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
            println!("[ERROR]: The run flag (-r, --run) must be ran with compilation mode (-c, --com).");
            std::process::exit(1);
        }
        true
    } else {false};

    let debug_flag = cl_args.contains(&CommandLineArgKind::Debug); 

    let asm_file: Option<&String> = asm_files.first();
    let out_file: Option<&String> = out_files.first();
    let obj_file: Option<String> = if let Some(f) = &asm_file { 
        if f.ends_with(".asm") { 
            Some( f.replace(".asm", ".o") )
        } else { 
            println!("[ERROR]: {} Specified ASM file output must have '.asm' suffix.", f); 
            std::process::exit(1);
        }
    } else {None};
    let obj_file: Option<&String> = obj_file.as_ref();
    


    let cl_args = CommandLineArgs::new(
        src_files,
        asm_file,
        out_file,
        obj_file,
        com_flag,
        debug_flag,
        run_com_flag,
        inc_dirs,
    );
    
    use std::path::{PathBuf, Path};
    let callers_dir = std::env::current_dir().unwrap();
    let path_buf = PathBuf::from(&cl_args.asm_file);
    let asm_obj_dir = path_buf.parent().unwrap(); 
    let path_buf = PathBuf::from(&cl_args.out_file);
    let out_dir = path_buf.parent().unwrap();
    let mut dirs_to_check: Vec<_> = Vec::new();
    for p in out_dir.ancestors() {
        dirs_to_check.push(p);    
    }
    for p in asm_obj_dir.ancestors() {
        dirs_to_check.push(p);    
    }
    dirs_to_check.sort();
    dirs_to_check.dedup();
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
                    _ => println!("[ERROR]: Entered {:?} is not a valid option.", input_buffer)
                }
            }
        }
    }

    cl_args
}

