pub trait LogFuncTrait<T: ToString>: Fn(T) -> String + Send + Sync + 'static {}
impl<F, T> LogFuncTrait<T> for F
where
    F: Fn(T) -> String + Send + Sync + 'static,
    T: ToString,
{
}
