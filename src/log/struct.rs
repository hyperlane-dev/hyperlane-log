#[derive(Clone)]
pub struct Log {
    pub(super) path: String,
    pub(super) limit_file_size: usize,
}
