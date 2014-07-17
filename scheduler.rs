
extern crate core;
extern crate collections;

use core::prelude::*;

use arch::thread::Thread;
use arch::thread;
use panic::{panic, println};
use allocator::malloc;

struct Scheduler {
  //queue: Deque<thread::Thread>,
  running: thread::Thread
}

impl Scheduler {
  
  pub fn new() -> Scheduler {
    Scheduler { running: Thread::current_state() }
  }
  
  pub fn schedule(&mut self, thread: thread::Thread) {
    //self.queue.push_back(thread);
  }
  
  pub fn timer_callback(&mut self) {
    /*self.queue.push_back(thread::Thread::current_state());
    match self.queue.pop_front() {
      Some(thread) => self.running = thread,
      None => panic()
    }
    unsafe { self.running.resume(); }*/
  }

}

extern "C" fn foo() {
  println("in a thread!");
}

pub fn thread_stuff() {
  println("starting thread test");
  let mem = malloc(1024*10);
  let t = Thread::new(foo, mem);
  let mut s = Scheduler::new();
  s.schedule(t);
  s.schedule(Thread::current_state());
  s.timer_callback();
  s.timer_callback();
}