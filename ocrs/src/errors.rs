use std::error::Error;
use std::fmt;
/// The error type returned when running a machine learning model fails.
#[derive(Debug)]
pub enum ModelRunError {
    /// Model execution failed.
    RunFailed(Box<dyn Error + Send + Sync>),
    /// The model output had a different data type or shape than expected.
    WrongOutput(String),
}
impl fmt::Display for ModelRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        panic!("STUB: not implemented");
    }
}
impl Error for ModelRunError {}
