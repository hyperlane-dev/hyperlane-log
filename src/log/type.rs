use crate::*;

pub type ListLog<T> = Vec<(String, ArcLogFunc<T>)>;
pub type LogListArcLock<T> = Arc<RwLock<ListLog<T>>>;
pub type LogArcLock = Arc<RwLock<Log>>;
pub type LogFunc<T> = dyn LogFuncTrait<T>;
pub type ArcLogFunc<T> = Arc<LogFunc<T>>;
pub type ArcLog = Arc<Log>;
