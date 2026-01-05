pub mod v8;

pub struct Runtime {
    engine: v8::V8Engine,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            engine: v8::V8Engine::new(),
        }
    }

    pub fn run_file(&mut self, path: std::path::PathBuf) {
        self.engine.run_file(path);
    }
}
