use crate::*;

#[derive(Clone, Data)]
pub struct Log {
    pub(super) path: String,
    pub(super) limit_file_size: usize,
}
