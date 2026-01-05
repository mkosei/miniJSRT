use std::path::Path;
use std::fs;
use rusty_v8 as v8;

pub struct V8Engine {
    isolate: v8::OwnedIsolate,
}

impl V8Engine {
    pub fn new() -> Self {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = v8::Isolate::new(Default::default());
        Self { isolate }
    }

    pub fn run_file(&mut self, filename: std::path::PathBuf) {
        if !Path::new(&filename).exists() {
            eprintln!("Error: File '{}' does not exist.", filename.display());
            return;
        }
        let source = fs::read_to_string(filename)
            .expect("Failed to read the source file.");

        self.run_source(&source);



    }

    fn run_source(&mut self, source: &str) {
        let isolate = &mut self.isolate;
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let global = context.global(scope);

        let print_fn = v8::Function::new(scope, print_callback).unwrap();
        let name = v8::String::new(scope, "print").unwrap();

        global.set(scope, name.into(), print_fn.into());

        let source = v8::String::new(scope, source).unwrap();

        let script = v8::Script::compile(scope, source, None).unwrap();
        script.run(scope).unwrap();
    }
}   

fn print_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let msg = args.get(0).to_rust_string_lossy(scope);
    println!("{}", msg);
}