use std::env;
use std::fs;
use  rusty_v8 as v8;
fn main() {

    let filename = env::args().nth(1).expect("js file required");
    let code = fs::read_to_string(filename).expect("failed to read file");

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);


    let global = context.global(scope);

    let print_fn = v8::Function::new(scope, print_callback).unwrap();
    let name = v8::String::new(scope, "print").unwrap();

    global.set(scope, name.into(), print_fn.into());

    let source = v8::String::new(scope, &   code).unwrap();

    let script = v8::Script::compile(scope, source, None).unwrap();
    script.run(scope).unwrap();


}

fn print_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let msg = args.get(0).to_rust_string_lossy(scope);
    println!("{}", msg);
}
