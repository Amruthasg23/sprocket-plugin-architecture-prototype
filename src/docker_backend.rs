use crate::plugin_interface::ExecutionBackend;

pub struct DockerBackend;

impl ExecutionBackend for DockerBackend {
    fn name(&self) -> &str {
        "DockerBackend"
    }

    fn execute(&self, job: &str) {
        println!("Executing job '{}' inside Docker container", job);
    }
}