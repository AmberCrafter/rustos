pub mod simple_executor;

use core::{pin::Pin, future::Future, task::{Context, Poll}};
use alloc::boxed::Box;


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