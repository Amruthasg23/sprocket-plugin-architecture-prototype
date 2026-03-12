mod plugin_interface;
mod sample_backend;
mod docker_backend;
mod remote_backend;

use plugin_interface::ExecutionBackend;
use sample_backend::LocalBackend;
use docker_backend::DockerBackend;
use remote_backend::RemoteBackend;

fn main() {

    let backends: Vec<Box<dyn ExecutionBackend>> = vec![
        Box::new(LocalBackend),
        Box::new(DockerBackend),
        Box::new(RemoteBackend),
    ];

    for backend in backends {
        println!("Loaded backend: {}", backend.name());
        backend.execute("example_job");
        println!("-----------------------");
    }
}
