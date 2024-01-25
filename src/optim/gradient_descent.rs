use derive_builder::Builder;
use optim::Optimize;

#[derive(Debug, Clone, Builder, Default)]
pub struct GradientDescent {
    pub learning_rate: f64,
    pub momentum: f64,
    pub nesterov: bool,
}

impl GradientDescent {
    pub fn new(learning_rate: f64, momentum: f64, nesterov: bool) -> Self {
        Self {
            learning_rate,
            momentum,
            nesterov,
        }
    }

    pub fn default() -> Self {
        Self {
            learning_rate: 0.01,
            momentum: 0.0,
            nesterov: false,
        }
    }
}

impl Optimize for GradientDescent {
    fn step(&mut self, params: &mut Vec<f64>, grads: &Vec<f64>) {
        let mut velocity = vec![0.0; params.len()];
        for (i, (param, grad)) in params.iter_mut().zip(grads.iter()).enumerate() {
            velocity[i] = self.momentum * velocity[i] - self.learning_rate * grad;
            if self.nesterov {
                *param += self.momentum * velocity[i] - self.learning_rate * grad;
            } else {
                *param += velocity[i];
            }
        }
    }

    fn zero_grad(&mut self) {}
}
