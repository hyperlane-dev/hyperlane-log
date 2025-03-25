use crate::*;

pub type ListLog = Vec<(String, ArcLogFunc)>;
pub type LogListArcLock = Arc<RwLock<ListLog>>;
pub type LogArcLock<'a> = Arc<RwLock<Log<'a>>>;
pub type LogFunc = dyn LogFuncTrait;
pub type ArcLogFunc = Arc<LogFunc>;
pub type ArcLog<'a> = Arc<Log<'a>>;

#[derive(Clone, Lombok)]
pub struct Log<'a> {
    pub(super) path: &'a str,
    pub(super) limit_file_size: usize,
}
