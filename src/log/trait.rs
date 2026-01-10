/// Trait for log formatting functions.
///
/// Defines the interface for functions that format log data.
/// Implemented automatically for any compatible Fn(T) -> String.
///
/// # Generic Parameters
///
/// - `AsRef<str>` - The type of data to be formatted, which will be converted to string slice.
pub trait ServerLogFuncTrait<T: AsRef<str>>: Fn(T) -> String + Send + Sync {}
