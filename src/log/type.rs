use super::r#trait::LogFuncTrait;
use http_type::*;
use lombok_macros::Lombok;

pub type LogListArcLock = ArcRwLock<Vec<(String, BoxLogFunc)>>;
pub type LogArcLock = ArcRwLock<Log>;
pub type LogFunc = dyn LogFuncTrait;
pub type BoxLogFunc = Box<LogFunc>;

#[derive(Debug, Clone, Lombok)]
pub struct Log {
    pub(super) path: String,
    pub(super) file_size: usize,
}
