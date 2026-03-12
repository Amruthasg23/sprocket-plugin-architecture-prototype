use crate::plugin_interface::ExecutionBackend;

pub struct LocalBackend;

impl ExecutionBackend for LocalBackend {
    fn name(&self) -> &str {
        "LocalBackend"
    }

    fn execute(&self, job: &str) {
        println!("Executing job '{}' on LocalBackend", job);
    }
}