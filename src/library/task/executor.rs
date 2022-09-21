use core::task::{Context, Poll, Waker};

use alloc::{collections::BTreeMap, sync::Rc, task::Wake};
use crossbeam_queue::ArrayQueue;

use super::{Task, TaskId};

const TASK_QUEUE_SIZE: usize = 100;

struct TaskWaker {
    task_id: TaskId,
    task_queue: Rc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Rc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Rc::new(TaskWaker{
            task_id, task_queue
        }))
    }

    fn wake_task(&self) {
        self.task_queue
            .push(self.task_id)
            .expect("[Warning] task::executor::TaskWaker::wake_task task_queue is full")
    }
}

// These implements will be trigger by poll (StateMachine)
impl Wake for TaskWaker {
    fn wake(self: Rc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Rc<Self>) {
        self.wake_task();
    }
}

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Rc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Rc::new(ArrayQueue::new(TASK_QUEUE_SIZE)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("[Error] task::executor::Excutor::spawn tasks with the same ID is already in the tasks");
        }
        self.task_queue
            .push(task_id)
            .expect("[Warning] task::executor::Excutor::spawn task_queue is full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }

    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        interrupts::disable();
        if self.tasks.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}
