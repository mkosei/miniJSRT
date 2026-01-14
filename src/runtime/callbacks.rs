use rusty_v8 as v8;

pub fn print_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let msg = args.get(0).to_rust_string_lossy(scope);
    println!("{}", msg);
}

pub fn fetch_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let url = args.get(0).to_rust_string_lossy(scope);
    // Here you would normally perform an HTTP request.
    // For simplicity, we'll just return a mock response.
    let response = format!("Fetched data from {}", url);
    let v8_response = v8::String::new(scope, &response).unwrap();
    _retval.set(v8_response.into());
}