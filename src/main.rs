use rusty_v8 as v8;
use std::io::{self, Write};
use std::fs;

fn execute_js_code(isolate: &mut v8::Isolate, code: &str) {
    let handle_scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let code = v8::String::new(scope, code).unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    if let Some(result) = script.run(scope) {
        let result_str = result.to_string(scope).unwrap();
        println!("{}", result_str.to_rust_string_lossy(scope));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Initialize V8 platform and isolate safely
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let mut isolate = v8::Isolate::new(Default::default());

        if args.len() == 1 {
            // REPL mode
            loop {
                print!("\n> ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if input.trim() == "exit" {
                    break;
                }

                execute_js_code(&mut isolate, &input);
            }
        } else {
            // Execute JS file
            let file_path = &args[1];
            let code = fs::read_to_string(file_path).expect("Unable to read file");
            execute_js_code(&mut isolate, &code);
        }
    }

    // Dispose V8 resources in the correct order
    unsafe { v8::V8::dispose() };
    v8::V8::shutdown_platform();
}
