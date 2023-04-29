mod docker;
pub mod docker2;

pub trait Environment {
    fn setup(&mut self);
    fn run_script(&mut self, cmd: &str);
    fn cleanup(&mut self);
}
