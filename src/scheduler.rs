use std::one::{ONCE_INIT, Once};

use std::ty::Unsafe;
use std::collections::Deque;
use std::collections::dlist::DList;
use std::mem::transmute;

use arch::thread::{Thread, save_context, restore_context};
use panic::*;
use allocator::malloc;

struct Scheduler<'a> {
  queue: Box<Deque<Thread> + 'a>
}

lazy_static! {
  static ref SCHEDULER: Unsafe<Scheduler<'static>> = Unsafe::new(Scheduler::new());
}
  
impl<'a> Scheduler<'a> {
  
  pub fn new<'a>() -> Scheduler<'a> {
    let list: DList<Thread> = DList::new();
    Scheduler { queue: box list as Box<Deque<Thread>> }
  }
  
  pub fn schedule(&mut self, func: extern "C" fn() -> ()) {
    // TODO(ryan) need to add cleanup code to end of called function
    // or else top-level will return to nowhere
    const stack_size: uint = 1024 * 1024;
    let mem = box [0,..stack_size];
    let addr: uint = unsafe { transmute(mem) };
    debug!("stack is at 0x{:x}", addr)
    let t = Thread::new(func, mem, addr  + stack_size);
    self.queue.push_back(t);
  }
  
  fn unschedule_current(&mut self) {
    let t = self.queue.pop_front().unwrap();
    debug!("unscheduling")
    unsafe { restore_context(&t) }
  }
  
  pub fn switch(&mut self) {
    let mut saved = Thread::empty();
    let resumed = unsafe { save_context(&mut saved) };
    
    if resumed {
        debug!("resumed!");
    } else {
        self.queue.push_back(saved);
        let t = self.queue.pop_front().unwrap();
        t.debug();
        unsafe { restore_context(&t) };
    }
  }
  

}

fn inner_thread_test(arg: uint) {
  debug!("arg is {:u}", arg)
}

extern "C" fn test_thread() {
  inner_thread_test(11);
  unsafe {
    let s: *mut Scheduler = SCHEDULER.get();
    debug!("got sched 0x{:x}", s as u32)    
    (*s).unschedule_current(); 
  }
}

pub fn thread_stuff() {
  debug!("starting thread test");
  unsafe {
    static mut o: Once = ONCE_INIT;
    o.doit(|| { });
    let s: *mut Scheduler = SCHEDULER.get();
    let s2: *mut Scheduler = SCHEDULER.get();
    
    debug!("orig sched 0x{:x}", s as u32)
    //loop {};
    (*s).schedule(test_thread);
    (*s).switch();
    debug!("back")
  }
}
