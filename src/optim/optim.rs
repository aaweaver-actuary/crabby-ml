// Base optimizer trait used to implement all optimizers.

pub trait Optimize {
    pub fn new() -> Self;
    pub fn default() -> Self;
    pub fn step(&mut self, params: &mut Vec<f64>, grads: &Vec<f64>);
    pub fn zero_grad(&mut self);
}
