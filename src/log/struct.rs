use crate::*;

#[derive(Clone, Lombok)]
pub struct Log {
    pub(super) path: String,
    pub(super) limit_file_size: usize,
}
