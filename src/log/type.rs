use super::r#trait::LogFuncTrait;
use http_type::*;
use lombok_macros::Lombok;
use std::sync::Arc;

pub type ListLog = Vec<(String, ArcLogFunc)>;
pub type LogListArcLock = ArcRwLock<ListLog>;
pub type LogArcLock = ArcRwLock<Log>;
pub type LogFunc = dyn LogFuncTrait;
pub type ArcLogFunc = Arc<LogFunc>;

#[derive(Debug, Clone, Lombok)]
pub struct Log {
    pub(super) path: String,
    pub(super) file_size: usize,
    pub(super) interval_millis: usize,
}
