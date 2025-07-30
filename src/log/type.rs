use crate::*;

/// A collection of named log formatting functions.
pub type ListLog<T> = Vec<(String, ArcLogFunc<T>)>;
/// Thread-safe shared reference to a collection of log functions.
pub type LogListArcLock<T> = Arc<RwLock<ListLog<T>>>;
/// Thread-safe shared reference to a Log configuration instance.
pub type LogArcLock = Arc<RwLock<Log>>;
/// Trait object representing a log formatting function.
pub type LogFunc<T> = dyn LogFuncTrait<T>;
/// Thread-safe shared reference to a log formatting function.
pub type ArcLogFunc<T> = Arc<LogFunc<T>>;
/// Thread-safe shared reference to a Log configuration.
pub type ArcLog = Arc<Log>;
