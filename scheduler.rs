use core::one::{ONCE_INIT, Once};

use core::prelude::*;
use core::ty::Unsafe;
use collections::Deque;
use collections::dlist::DList;
use alloc::owned::Box;

use arch::thread::Thread;
use panic::*;
use allocator::malloc;

struct Scheduler {
  queue: Box<Deque<Thread>>
}

lazy_static! {
  static ref SCHEDULER: Unsafe<Scheduler> = Unsafe::new(Scheduler::new());
}
  
impl Scheduler {
  
  pub fn new() -> Scheduler {
    let list: DList<Thread> = DList::new();
    Scheduler { queue: box list as Box<Deque<Thread>> }
  }
  
  pub fn schedule(&mut self, func: extern "C" fn() -> ()) {
    // TODO(ryan) need to add cleanup code to end of called function
    // or else top-level will return to nowhere
    let mem = malloc(1024*10);
    let t = Thread::new(func, mem);
    self.queue.push_back(t);
  }
  
  pub fn switch(&mut self) {
    let thread = Thread::current_state_or_resumed();
    // TODO(ryan) my confidence in this is shaky because it relies on being able
    // to return to the same stack twice with different return values from current_state_or_resumed()
    // but the something might have gotten clobbered in the meantime that messes stuff up
    match thread {
      Some(thread) => {self.queue.push_back(thread) },
      None => { return }
    }

    match self.queue.pop_front() {
      Some(thread) => unsafe { thread.resume(); },
      None => panic_message("nothing in the queue?!")
    }
    
  }
  
  fn unschedule_current(&mut self) {
    match self.queue.pop_front() {
      Some(thread) => unsafe { thread.resume(); },
      None => panic_message("nothing in the queue?!")
    }
  }

}

impl Share for Scheduler { }

fn inner_thread_test(arg: uint) {
  print("    got int: "); put_int(arg as u32); println("");
}

extern "C" fn test_thread() {
  println("in a thread!");
  inner_thread_test(10);
  
  unsafe { (*SCHEDULER.get()).unschedule_current(); }
}

pub fn thread_stuff() {
  println("starting thread test");
  unsafe {
    static mut o: Once = ONCE_INIT;
    o.doit(|| {print("hi");});
    let s: *mut Scheduler = SCHEDULER.get();
    (*s).schedule(test_thread);
    (*s).switch();
  }
}