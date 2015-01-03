// TODO(ryan): it really looks like bulk of libgreen could be used here where pthread <-> core

use core::prelude::*;
//use std::one::{ONCE_INIT, Once};
use core::cell::UnsafeCell;
use core::mem::transmute;

use alloc::boxed::Box;

use collections::RingBuf;
use collections::dlist::DList;


use thread::context::Context;
use thread::stack::Stack;

struct Tcb { // thread control block
  context: Context,
  _stack: Stack
}

struct Scheduler<'a> { // invariant: current thread is at back of queue
  queue: Box<RingBuf<Tcb> + 'a>
}

lazy_static_spin! {
  static SCHEDULER: UnsafeCell<Scheduler<'static>> = UnsafeCell::new(Scheduler::new());
}

extern "C" fn run_thunk(sched: uint, code: *mut (), env: *mut ()) -> ! {
  let scheduler: &mut Scheduler = unsafe { transmute(sched) };
  let c: |&mut Scheduler| -> () = unsafe { transmute((code, env)) };
  c(scheduler);
  warn!("didn't unschedule finished thread");
  unreachable!();
}

extern "C" fn dummy() {
  // TODO: eventually remove this and put the kernel in the scheduler as well
} 

impl<'a> Scheduler<'a> {
  
  pub fn new<'a>() -> Scheduler<'a> {
    let list: DList<Tcb> = DList::new();
    let mut s = Scheduler { queue: box list as Box<RingBuf<Tcb>> };
    let dummy_tcb = s.new_tcb(dummy);
    s.queue.push(dummy_tcb); // put a dummy at back of queue and pretend it's current thread
    s
  }
  
  pub fn schedule(&mut self, func: extern "C" fn() -> ()) {
    let current = self.queue.pop().unwrap();
    let new_tcb = self.new_tcb(func);
    self.queue.push(new_tcb);
    self.queue.push(current);
  }
  
  fn new_tcb(&self, func: extern "C" fn() -> ()) -> Tcb {
    const STACK_SIZE: uint = 1024 * 1024;
    let mut stack = Stack::new(STACK_SIZE);

    let p  = move |:scheduler: &mut Scheduler| {
      func();
      scheduler.unschedule_current();
    };
    
    let c = Context::new(run_thunk, unsafe { transmute(&self) }, unsafe{ transmute(p)}, &mut stack);
    Tcb { context: c, _stack: stack }
  }
  
  fn unschedule_current(&mut self) {
    let t = self.queue.pop_front().unwrap();
    debug!("unscheduling");
    let mut dont_care = Context::empty();
    Context::swap(&mut dont_care, &t.context);
  }
  
  pub fn switch(&mut self) {
    let next = self.queue.pop_front().unwrap();
    let old_context = &mut self.queue.back_mut().unwrap().context;
    Context::swap(old_context, &next.context);    
  }
  
}

fn inner_thread_test(arg: uint) {
  debug!("arg is {}", arg)
}

extern "C" fn test_thread() {
  debug!("in a test thread!");
  inner_thread_test(11);
  unsafe {
    let s: *mut Scheduler = SCHEDULER.get();
    debug!("leaving test thread!")    
      (*s).unschedule_current(); 
  }
}

pub fn thread_stuff() {
  debug!("starting thread test");
  unsafe {
    let s: *mut Scheduler = SCHEDULER.get();

    debug!("orig sched 0x{:x}", s as u32)
      //loop {};
      (*s).schedule(test_thread);
    (*s).switch();
    debug!("back")
  }
}
