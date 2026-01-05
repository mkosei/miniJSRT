pub mod run;
pub mod server;

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "run" => run::command(&args[2..]),
        "server" => server::command(&args[2..]),
        _ => print_help(),
        
    }

}

fn print_help() {
    eprintln!("Usage:");
    eprintln!("  mini-runtime run <file.js>");
    eprintln!("  mini-runtime serve");
}
