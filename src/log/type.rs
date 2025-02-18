use crate::*;

pub type ListLog = Vec<(String, ArcLogFunc)>;
pub type LogListArcLock = Arc<RwLock<ListLog>>;
pub type LogArcLock = Arc<RwLock<Log>>;
pub type LogFunc = dyn LogFuncTrait;
pub type ArcLogFunc = Arc<LogFunc>;

#[derive(Debug, Clone, Lombok)]
pub struct Log {
    pub(super) path: String,
    pub(super) file_size: usize,
    pub(super) interval_millis: usize,
}
