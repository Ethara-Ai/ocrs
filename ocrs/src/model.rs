use anyhow::anyhow;
use rten_tensor::{Tensor, TensorView};
/// Interface for running an ML model.
pub trait Model {
    /// Return the expected input shape as a mix of fixed and dynamic-sized
    /// dimensions.
    fn input_shape(&self) -> anyhow::Result<Vec<rten::Dimension>>;
    /// Run the model and return inference outputs.
    fn run(
        &self,
        input: TensorView<f32>,
        opts: Option<rten::RunOptions>,
    ) -> anyhow::Result<Tensor<f32>>;
}
impl Model for rten::Model {
    fn input_shape(&self) -> anyhow::Result<Vec<rten::Dimension>> {
        panic!("STUB: not implemented");
    }
    fn run(
        &self,
        input: TensorView<f32>,
        opts: Option<rten::RunOptions>,
    ) -> anyhow::Result<Tensor<f32>> {
        panic!("STUB: not implemented");
    }
}
