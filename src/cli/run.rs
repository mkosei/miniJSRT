use std::path::Path;

use crate::runtime::Runtime;

pub fn command(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No file specified.");
        return;
    }
    let filename = &args[0];
    if !Path::new(filename).exists() {
        eprintln!("Error: File '{}' does not exist.", filename);
        return;
    }

    let mut runtime = Runtime::new();
    runtime.run_file(filename.into());

}