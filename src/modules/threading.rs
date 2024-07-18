use std::thread::JoinHandle;
use super::err::MiruErr;
pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>
}

impl ThreadPool{
    pub fn new(size: usize) -> Result<ThreadPool, MiruErr> {
        if size == 0 {
            let e: &str = "Thread number cannot be zero!";
            return Err::<ThreadPool, MiruErr>(MiruErr::new(&e.to_string()));
        }
        else {
            let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(size);
            for i in 0..size {

            }
            Ok(ThreadPool { threads: threads })
        }
    }
}