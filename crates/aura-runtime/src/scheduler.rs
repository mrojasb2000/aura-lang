//! M:N Green Thread (Fiber) Scheduler with cooperative yielding and parking.

use crate::context::aura_swap_context;
use crate::fiber::{Fiber, FiberState};
use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

thread_local! {
    static CURRENT_FIBER: Cell<*mut Fiber> = const { Cell::new(std::ptr::null_mut()) };
    static SCHEDULER_SP: Cell<*mut *mut u8> = const { Cell::new(std::ptr::null_mut()) };
}

pub struct Scheduler {
    ready_queue: Arc<Mutex<VecDeque<Box<Fiber>>>>,
    all_fibers: Arc<Mutex<Vec<usize>>>,
    cond: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
}

use std::sync::OnceLock;

static GLOBAL_SCHEDULER: OnceLock<Scheduler> = OnceLock::new();

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            all_fibers: Arc::new(Mutex::new(Vec::new())),
            cond: Arc::new(Condvar::new()),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn spawn(&self, entry: extern "C" fn(*mut ()), arg: *mut ()) -> u64 {
        let fiber = Box::new(Fiber::new(entry, arg, fiber_entry_trampoline));
        let id = fiber.id;
        let raw = Box::into_raw(fiber);

        {
            let mut all = self.all_fibers.lock().unwrap();
            all.push(raw as usize);
        }

        {
            let mut q = self.ready_queue.lock().unwrap();
            q.push_back(unsafe { Box::from_raw(raw) });
            self.cond.notify_one();
        }

        id
    }

    pub fn run_workers(&'static self, num_workers: usize) {
        let mut handles = Vec::new();
        for _ in 1..num_workers {
            let ready_q = Arc::clone(&self.ready_queue);
            let cond = Arc::clone(&self.cond);
            let shutdown = Arc::clone(&self.shutdown);

            handles.push(thread::spawn(move || {
                worker_loop(ready_q, cond, shutdown);
            }));
        }

        // Main thread acts as worker 0
        worker_loop(
            Arc::clone(&self.ready_queue),
            Arc::clone(&self.cond),
            Arc::clone(&self.shutdown),
        );

        for h in handles {
            let _ = h.join();
        }
    }
}

pub fn init_scheduler() {
    let _ = GLOBAL_SCHEDULER.get_or_init(Scheduler::new);
}

pub fn get_scheduler() -> &'static Scheduler {
    GLOBAL_SCHEDULER.get_or_init(Scheduler::new)
}

fn worker_loop(
    ready_q: Arc<Mutex<VecDeque<Box<Fiber>>>>,
    cond: Arc<Condvar>,
    shutdown: Arc<AtomicBool>,
) {
    let mut worker_sp: *mut u8 = std::ptr::null_mut();

    loop {
        let mut fiber = {
            let mut q = ready_q.lock().unwrap();
            while q.is_empty() {
                if shutdown.load(Ordering::Relaxed) {
                    return;
                }
                q = cond.wait(q).unwrap();
            }
            q.pop_front().unwrap()
        };

        if fiber.state == FiberState::Dead {
            continue;
        }

        fiber.state = FiberState::Running;
        let raw_fiber = Box::into_raw(fiber);

        CURRENT_FIBER.set(raw_fiber);
        SCHEDULER_SP.set(&mut worker_sp);

        unsafe {
            let fiber_sp = (*raw_fiber).sp;
            aura_swap_context(&mut worker_sp, fiber_sp);
        }

        // Resumed back in worker thread stack!
        let fiber = unsafe { Box::from_raw(raw_fiber) };
        CURRENT_FIBER.set(std::ptr::null_mut());

        match fiber.state {
            FiberState::Ready => {
                let mut q = ready_q.lock().unwrap();
                q.push_back(fiber);
                cond.notify_one();
            }
            FiberState::Blocked => {
                // Fiber parked on channel or I/O, keep leaked until unparked
                let _ = Box::into_raw(fiber);
            }
            FiberState::Dead => {
                // Fiber completed, clean up
                let mut all = get_scheduler().all_fibers.lock().unwrap();
                all.retain(|&f| f != raw_fiber as usize);
                drop(fiber);
            }
            FiberState::Running => unreachable!(),
        }

        let q = ready_q.lock().unwrap();
        if q.is_empty() {
            let all = get_scheduler().all_fibers.lock().unwrap();
            if all.is_empty() {
                shutdown.store(true, Ordering::Relaxed);
                cond.notify_all();
                return;
            }
        }
    }
}

extern "C" fn fiber_entry_trampoline() -> ! {
    let raw = CURRENT_FIBER.get();
    if !raw.is_null() {
        let (entry_fn, arg) = unsafe { ((*raw).entry_fn.take(), (*raw).entry_arg) };
        if let Some(f) = entry_fn {
            f(arg);
        }
    }

    yield_exit();
}

pub fn yield_now() {
    let raw = CURRENT_FIBER.get();
    if raw.is_null() {
        return;
    }

    unsafe {
        (*raw).state = FiberState::Ready;
        let worker_sp_ptr = SCHEDULER_SP.get();
        if !worker_sp_ptr.is_null() {
            aura_swap_context(&mut (*raw).sp, *worker_sp_ptr);
        }
    }
}

pub fn park_current() {
    let raw = CURRENT_FIBER.get();
    if raw.is_null() {
        return;
    }

    unsafe {
        (*raw).state = FiberState::Blocked;
        let worker_sp_ptr = SCHEDULER_SP.get();
        if !worker_sp_ptr.is_null() {
            aura_swap_context(&mut (*raw).sp, *worker_sp_ptr);
        }
    }
}

pub fn unpark_fiber(raw_fiber: *mut Fiber) {
    if raw_fiber.is_null() {
        return;
    }

    let fiber = unsafe {
        (*raw_fiber).state = FiberState::Ready;
        Box::from_raw(raw_fiber)
    };

    let sched = get_scheduler();
    let mut q = sched.ready_queue.lock().unwrap();
    q.push_back(fiber);
    sched.cond.notify_one();
}

pub fn current_fiber_ptr() -> *mut Fiber {
    CURRENT_FIBER.get()
}

pub fn for_each_active_fiber<F: FnMut(*const u8, *const u8)>(mut f: F) {
    let sched = get_scheduler();
    let all = match sched.all_fibers.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for &fiber_addr in all.iter() {
        if fiber_addr != 0 {
            let fiber = unsafe { &*(fiber_addr as *const Fiber) };
            f(fiber.sp, fiber.stack.top());
        }
    }
}

fn yield_exit() -> ! {
    let raw = CURRENT_FIBER.get();
    unsafe {
        if !raw.is_null() {
            (*raw).state = FiberState::Dead;
            let worker_sp_ptr = SCHEDULER_SP.get();
            if !worker_sp_ptr.is_null() {
                aura_swap_context(&mut (*raw).sp, *worker_sp_ptr);
            }
        }
    }
    unreachable!("Fiber exited but resumed execution");
}
