use crate::runtime::v8::V8Engine;
use tokio::time::{sleep, Duration};

pub async fn event_loop(engine: &V8Engine) {
    loop {
        while let Some(task) = engine.get_task_queue().lock().unwrap().pop_front() {
            task(&mut engine.isolate_handle_scope());
        }

        engine.platform_microtask();
        sleep(Duration::from_millis(1)).await;
    }
}