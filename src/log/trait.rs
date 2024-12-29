pub trait LogFuncTrait: Fn(&String) -> String + Send + Sync + 'static {}
impl<T> LogFuncTrait for T where T: Fn(&String) -> String + Send + Sync + 'static {}
pub trait LogDataTrait: Into<String> {}
impl<T> LogDataTrait for T where T: Into<String> + 'static {}
