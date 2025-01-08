use crate::{Log, LogArcLock};
use lazy_static::lazy_static;
use std::{
    any::Any,
    panic::{self, AssertUnwindSafe},
    sync::{Arc, Mutex, RwLock},
    thread::{spawn, JoinHandle},
};

lazy_static! {
    static ref LOG: LogArcLock = Arc::new(RwLock::new(Log::default()));
}

fn restart_thread<F>(f: F) -> JoinHandle<()>
where
    F: Fn() + Send + 'static,
{
    let f: Arc<Mutex<F>> = Arc::new(Mutex::new(f));
    spawn(move || {
        let _: Result<(), Box<dyn Any + Send>> = panic::catch_unwind(AssertUnwindSafe({
            let f: Arc<Mutex<F>> = f.clone();
            move || {
                if let Ok(func) = f.lock() {
                    func();
                }
            }
        }));
    })
}

#[inline]
pub fn log_run(log: &Log) -> JoinHandle<()> {
    let log_clone: Log = log.clone();
    {
        if let Ok(mut log) = LOG.write() {
            *log = log_clone;
        }
    }
    let log_thread: JoinHandle<()> = restart_thread(move || {
        let error_log_thread: JoinHandle<()> = restart_thread(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_error();
            }
        });
        let info_log_thread: JoinHandle<()> = restart_thread(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_info();
            }
        });
        let debug_log_thread: JoinHandle<()> = restart_thread(move || loop {
            if let Ok(log) = LOG.read() {
                log.write_debug();
            }
        });
        let _ = error_log_thread.join();
        let _ = info_log_thread.join();
        let _ = debug_log_thread.join();
    });
    log_thread
}
