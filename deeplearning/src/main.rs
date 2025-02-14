use burn::{backend, prelude::*};
use nn::{
    conv::{Conv2d, Conv2dConfig},
    pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
    Gelu, Linear, LinearConfig,
};

#[derive(Module, Debug)]
struct Model<B: Backend> {
    conv1: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    linear: Linear<B>,
    activation: Gelu,
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    num_classes: usize,
    hidden_size: usize,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            conv1: Conv2dConfig::new([1, 8], [3, 3]).init(device),
            pool: AdaptiveAvgPool2dConfig::new([8, 8]).init(),
            linear: LinearConfig::new(16 * 8 * 8, self.num_classes).init(device),
            activation: Gelu::new(),
        }
    }
}

fn main() {
    type MyBackend = backend::Wgpu<f32, i32>;
    let device = Default::default();
    let model = ModelConfig::new(10, 512).init::<MyBackend>(&device);

    println!("{}", model)
}
