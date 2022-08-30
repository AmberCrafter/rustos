pub mod simple_executor;
pub mod keyboard;

use core::{pin::Pin, future::Future, task::{Context, Poll}};
use alloc::boxed::Box;

use self::simple_executor::SimpleExecutor;


pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future: Box::pin(future)
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}


pub fn init() {
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(keyboard::execute_keycode()));
    executor.run();
}