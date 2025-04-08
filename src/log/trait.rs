pub trait LogFuncTrait<T: ToString>: Fn(T) -> String + Send + Sync {}
impl<F, T> LogFuncTrait<T> for F
where
    F: Fn(T) -> String + Send + Sync,
    T: ToString,
{
}
