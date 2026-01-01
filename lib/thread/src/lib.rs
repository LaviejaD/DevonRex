use std::thread;
#[derive(Debug)]
pub struct Worker {
    thread: thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(t: thread::JoinHandle<()>) -> Self {
        Self { thread: t }
    }
    pub fn join(self) {
        let _ = self.thread.join();
    }
    pub fn is_free(&self) -> bool {
        self.thread.is_finished()
    }
}
pub struct ThreadManager {
    thread_usize: usize,
    workers: Vec<Worker>,
}

impl ThreadManager {
    pub fn new(thread_usize: usize) -> Self {
        ThreadManager {
            thread_usize,
            workers: Vec::new(),
        }
    }

    pub fn add(&mut self, th: thread::JoinHandle<()>) {
        self.workers.push(Worker::new(th));
        self.check();
    }
    pub fn join(&mut self) {
        while let Some(w) = self.workers.pop() {
            w.join();
        }
    }

    pub fn wait(&mut self) {
        let mut temp = Vec::new();
        temp.append(&mut self.workers);

        while let Some(w) = temp.pop() {
            if !w.is_free() {
                self.workers.push(w);
            } else {
                w.join();
            }
        }
    }

    pub fn check(&mut self) {
        if self.workers.len() >= self.thread_usize {
            self.wait();
        }
    }
}
