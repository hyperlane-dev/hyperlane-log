use crate::*;

pub type ListLog = Vec<(String, ArcLogFunc)>;
pub type LogListArcLock = Arc<RwLock<ListLog>>;
pub type LogArcLock = Arc<RwLock<Log>>;
pub type LogFunc = dyn LogFuncTrait;
pub type ArcLogFunc = Arc<LogFunc>;
pub type ArcLog = Arc<Log>;

#[derive(Clone, Lombok)]
pub struct Log {
    pub(super) path: String,
    pub(super) limit_file_size: usize,
}
