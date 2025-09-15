// install nightly channel: `rustup toolchain install nightly`
// set nightlly channel: `rustup override set nightly`
// !TODO: https://github.com/rust-lang/rust/issues/40180
#![feature(abi_x86_interrupt)]

pub mod interrupts {
    use conquer_once::spin;
    use x86_64::structures::idt::{
        InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
    };

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum InterruptIndex {
        Timer = PIC_1_OFFSET,
        Keyboard,
    }

    impl InterruptIndex {
        fn as_u8(self) -> u8 {
            self as u8
        }

        fn as_usize(self) -> usize {
            usize::from(self.as_u8())
        }
    }

    pub const PIC_1_OFFSET: u8 = 32;
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
    pub static PICS: spin::Mutex<ChainedPics> =
        spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

    #[allow(dead_code)]
    extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
        use x86_64::instructions::port::Port;
        let scancode: u8 = unsafe { port.read() };
        create::task::keyboard::add_scandcode(scancode);

        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    }
}

pub mod task {

    pub mod keyboard {
        impl Stream fro ScancodeStream {
            type Item = u8;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
                let queue = SCANCODE_QUEUE
                    .try_get().expect("scancode queue not initialized");

                if let Some(scancode) = queue.pop() {
                    return Poll::Ready(Some(scancode));
                }

                WAKER.register(&cx.waker());
                match queue.pop() {
                    Some(Scancode) => {
                        WAKER.take();
                        Poll::Ready(Some(scancode))
                    }
                    None => Poll::Pending
                }
            }
        }
        use conquer_once::spin::OnceCell;
        use crossbeam_queue::ArrayQueue;

        static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

        pub(crate) fn add_scandcode(scancode: u8) {
            if let Ok(queue) = SCANCODE_QUEUE.try_get() {
                if let Err(_) = queue.push(scancode) {
                    println!("WARNING: scancode queue full; dropping keyboard input");
                } else {
                    WAKER.wake();
                }
            } else {
                println!("WARNING: scancode queue not initialized yet");
            }
        }

        pub struct ScancodeStream {
            _private: (),
        }

        impl ScancodeStream {
            pub fn new() -> Self {
                SCANCODE_QUEUE
                    .try_init_once(|| ArrayQueue::new(100))
                    .expect("ScancodeStream::new should only be called once");
                ScancodeStream { _private: () }
            }
        }

        use core::{
            pin::Pin,
            task::{Context, Poll},
        };
        use futures_util::stream::Stream;

        impl Stream for ScancodeStream {
            type Item = u8;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
                let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
                match queue.pop() {
                    Some(scancode) => Poll::Ready(Some(scancode)),
                    None => Poll::Pending,
                }
            }
        }
    }

    pub mod simple_executor {
        use std::{
            collections::VecDeque,
            task::{Context, Poll, RawWakerVTable},
        };

        use crate::lib::Task;

        pub struct SimpleExecutor {
            task_queue: VecDeque<Task>,
        }

        impl SimpleExecutor {
            pub fn new() -> SimpleExecutor {
                SimpleExecutor {
                    task_queue: VecDeque::new(),
                }
            }

            pub fn spawn(&mut self, task: Task) {
                self.task_queue.push_back(task)
            }

            pub fn run(&mut self) {
                while let Some(mut task) = self.task_queue.pop_front() {
                    let waker = dummy_waker();
                    let mut context = Context::from_waker(&waker);
                    match task.poll(&mut context) {
                        Poll::Ready(()) => {}
                        Poll::Pending => self.task_queue.push_back(task),
                    }
                }
            }
        }

        use core::task::{RawWaker, Waker};

        fn dummy_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker {
                dummy_raw_waker()
            }

            let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), vtable)
        }

        fn dummy_waker() -> Waker {
            unsafe { Waker::from_raw(dummy_raw_waker()) }
        }
    }
}

pub mod lib {
    use core::{future::Future, pin::Pin};
    use std::task::{Context, Poll};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TaskId(u64);

    pub struct Task {
        future: Pin<Box<dyn Future<Output = ()>>>,
    }

    impl TaskId {
        fn new() -> Self {
            static NEXT_ID: AtomicU64 = AtomicU64::new(0);
            TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
        }
    }

    impl Task {
        pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
            Task {
                future: Box::pin(future),
            }
        }

        pub fn poll(&mut self, context: &mut Context) -> Poll<()> {
            self.future.as_mut().poll(context)
        }
    }
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("The number is {}", number);
}

// ala kernel_main
fn main() {
    let mut executor = task::simple_executor::SimpleExecutor::new();
    executor.spawn(lib::Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
