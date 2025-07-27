use crate::*;

/// A list of named log functions.
pub type ListLog<T> = Vec<(String, ArcLogFunc<T>)>;
/// Thread-safe shared reference to a list of log functions.
pub type LogListArcLock<T> = Arc<RwLock<ListLog<T>>>;
/// Thread-safe shared reference to a log instance.
pub type LogArcLock = Arc<RwLock<Log>>;
/// Trait object for log functions.
pub type LogFunc<T> = dyn LogFuncTrait<T>;
/// Shared reference to a log function trait object.
pub type ArcLogFunc<T> = Arc<LogFunc<T>>;
/// Shared reference to a log instance.
pub type ArcLog = Arc<Log>;
