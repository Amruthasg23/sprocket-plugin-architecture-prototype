pub trait ExecutionBackend {
    fn name(&self) -> &str;
    fn execute(&self, job: &str);
}