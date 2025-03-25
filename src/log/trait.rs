pub trait LogFuncTrait: Fn(&str) -> String + Send + Sync + 'static {}
impl<T> LogFuncTrait for T where T: Fn(&str) -> String + Send + Sync + 'static {}
