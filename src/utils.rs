pub fn print_usage(args: &Vec<String>) {
    let suffix = ".abc";
    println!("[ERROR]: Usage: {} <FLAG> [ARGS]", &args[0].as_str());
    println!("    FILES: Source files must end with {suffix} suffix");
    println!("    FLAGS: -s --simulation mode - interprets the code, doesnt create executable");
    println!("           -c --compilation mode (default) - generates assembly code, creates executable");
    println!("           -r --run - runs the generated executable after compilation");
    println!("           -o [executable_name] --output - specifies the executable path");
}