use crate::plugin_interface::ExecutionBackend;

pub struct RemoteBackend;

impl ExecutionBackend for RemoteBackend {
    fn name(&self) -> &str {
        "RemoteBackend"
    }

    fn execute(&self, job: &str) {
        println!("Executing job '{}' on Remote Server", job);
    }
}