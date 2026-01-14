use std::{collections::VecDeque, path::Path};
use std::fs;
use rusty_v8::{self as v8, OwnedIsolate};
use std::sync::{Arc, Mutex};

use crate::runtime::callbacks::{print_callback, fetch_callback};

pub type JsTask = Box<dyn FnOnce(&mut v8::HandleScope) + Send>;


pub struct V8Engine {
    isolate: v8::OwnedIsolate,
    tasks: Arc<Mutex<VecDeque<JsTask>>>,
}

impl V8Engine {
    pub fn new() -> Self {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = v8::Isolate::new(Default::default());
        Self { isolate, tasks: Arc::new(Mutex::new(VecDeque::new()))  }
    }

    pub fn enqueue_task(&self, task: JsTask) {
        self.tasks.lock().unwrap().push_back(task);
    }

    pub fn get_task_queue(&self) -> Arc<Mutex<VecDeque<JsTask>>> {
        Arc::clone(&self.tasks)
    }

    pub fn isolate_handle_scope(&mut self) -> &mut v8::HandleScope {
        // 新規 handle scope 作成して返す
        // ここでは簡易的に self.isolate を返す想定
        // 実際は ContextScope と組み合わせて使う
        todo!()
    }

    pub fn perform_microtasks(&mut self) {
        self.isolate.perform_microtask_checkpoint();
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

        let fetch_fn = v8::Function::new(scope, fetch_callback).unwrap();
        let fetch_name = v8::String::new(scope, "fetch").unwrap();

        global.set(scope, name.into(), print_fn.into());
        global.set(scope, fetch_name.into(), fetch_fn.into());

        let source = v8::String::new(scope, source).unwrap();

        let script = v8::Script::compile(scope, source, None).unwrap();
        script.run(scope).unwrap();
    }
}   

