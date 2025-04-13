pub trait LogFuncTrait<T: ToString>: Fn(T) -> String + Send + Sync {}
