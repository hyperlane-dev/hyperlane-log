pub(crate) mod cfg;
pub(crate) mod log;

pub use log::{r#trait::*, r#type::*, thread::log_run};
pub use std::thread::JoinHandle;
