/// Trait for log formatting functions.
///
/// Defines the interface for functions that format log data.
/// Implemented automatically for any compatible Fn(T) -> String.
///
/// # Generic Parameters
///
/// - `T: ToString` - The type of data to be formatted, must be convertible to String.
pub trait LogFuncTrait<T: ToString>: Fn(T) -> String + Send + Sync {}
