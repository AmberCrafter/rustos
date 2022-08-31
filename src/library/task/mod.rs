// pub mod simple_executor;
pub mod executor;
pub mod keyboard;

use core::{pin::Pin, future::Future, task::{Context, Poll}, sync::atomic::{AtomicU64, Ordering::Relaxed}};
use alloc::boxed::Box;

// use self::simple_executor::SimpleExecutor;
use self::executor::Executor;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self(NEXT_ID.fetch_add(1, Relaxed))
    }
}


pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future)
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}


pub fn init() {
    // let mut executor = SimpleExecutor::new();
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::execute_keycode()));
    executor.run();
}