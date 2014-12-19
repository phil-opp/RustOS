use std::prelude::*;
use std::one::{ONCE_INIT, Once};

use std::ty::Unsafe;
use collections::Deque;
use collections::dlist::DList;
use core::mem::transmute;

use arch::context::{Context, save_context, restore_context};

struct Scheduler<'a> {
  queue: Box<Deque<Context> + 'a>
}

lazy_static! {
  static ref SCHEDULER: Unsafe<Scheduler<'static>> = Unsafe::new(Scheduler::new());
}
  
impl<'a> Scheduler<'a> {
  
  pub fn new<'a>() -> Scheduler<'a> {
    let list: DList<Context> = DList::new();
    Scheduler { queue: box list as Box<Deque<Context>> }
  }
  
  pub fn schedule(&mut self, func: extern "C" fn() -> ()) {
    // TODO(ryan) need to add cleanup code to end of called function
    // or else top-level will return to nowhere
    const STACK_SIZE: uint = 1024 * 1024;
    let mem = box [0,..STACK_SIZE];
    let addr: uint = unsafe { transmute(mem) };
    debug!("stack is at 0x{:x}", addr)
    let t = Context::new(func, mem, addr  + STACK_SIZE);
    self.queue.push(t);
  }
  
  fn unschedule_current(&mut self) {
    let t = self.queue.pop_front().unwrap();
    debug!("unscheduling")
    unsafe { restore_context(&t) }
  }
  
  pub fn switch(&mut self) {
    let mut saved = Context::empty();
    let resumed = unsafe { save_context(&mut saved) };
    
    if resumed {
        debug!("resumed!");
    } else {
        self.queue.push(saved);
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

    debug!("orig sched 0x{:x}", s as u32)
    //loop {};
    (*s).schedule(test_thread);
    (*s).switch();
    debug!("back")
  }
}
