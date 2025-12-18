use std::{default, env::Args};


fn main(){
    let args: Vec<String> = std::env::args().collect();
	let mut show_log: bool = false;
	let mut show_exit_code: bool = false;
    
	for arg in &args{
        match arg.as_str() {
            "-info" => {
                println!("Baulu Compiler\na compiler for the baulu programming language");
                return ()
            },
            "-help" => {
                println!("Usage: baulo <source-file> [options]");
                println!("Options:");
                println!("\t-info:      Display information about the compiler");
                println!("\t-help:      Display this help message");
                println!("\t-log:       Enable detailed logging during compilation and execution");
                println!("\t-exit-code: Display the exit code for the program");
                println!("\t-version:   Display the compiler version");
                return ();
            },
            "-log" => {
                show_log = true;
            },
            "-version" => {
                println!("Baulu Compiler version 0.1.0");
                return ();
            },
            "-exit-code" => {
                show_exit_code = true;
            },
            default => (),
        }
	}

    if args.len()- show_exit_code as usize - show_log as usize <= 1 {
		println!("No File Included. for more info use \"-help\".");
		return ();
	}
}