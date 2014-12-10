use std::one::{ONCE_INIT, Once};

use std::ty::Unsafe;
use std::collections::Deque;
use std::collections::dlist::DList;
use std::mem::transmute;

use arch::thread::Thread;
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
    let mem = box [0,..1024 * 1024];
    let addr: u32 = unsafe { transmute(mem) };
    debug!("stack is at 0x{:x}", addr)
    let t = Thread::new(func, mem, addr);
    self.queue.push_back(t);
  }
  
  fn unschedule_current(&mut self) {
    let mut t = self.queue.pop_front().unwrap();
    debug!("unscheduling")
    unsafe {
      let callback = |old: Box<Thread>, new: &Thread| { 
	t.switch_to(None);
      };
      
      let mut s = Scheduler::switcher();
      s.switch_to(Some(transmute(&callback)))
    }
  }
  
  pub fn switch(&mut self) {
    let mut t = self.queue.pop_front().unwrap();
    debug!("switching"); 
    let ref mut q = self.queue;
    debug!("q at 0x{:x}", raw(q));
    
    unsafe {

    let mut s = Scheduler::switcher();
    let c = |old: Box<Thread>, new: &Thread| { 
      debug!("in closure");
      debug!("q at 0x{:x}", raw(q));
      //debug!("old at 0x{:x}", rawb(old));
      let o = *old;
      debug!("deref");
      q.push_back(o);
      debug!("pushed");
      //loop {}
       
      
      
      t.switch_to(None);
    };

    s.switch_to(Some(&c));
    }
  }
  
  fn switcher() -> Thread {
    let mem = box [0,..1024];
    let addr: u32 = unsafe { transmute(mem) };
    let t = Thread::new(unsafe { transmute(intermediate_handler) }, mem, addr);
    t
  }

}

fn raw<T>(p: &T) -> u32 {
  unsafe { transmute(p) }
}

fn rawb<T>(p: Box<T>) -> u32 {
  unsafe { transmute(p) }
}

extern "C" fn intermediate_handler(old_thread: Box<Thread>, new_thread: &Thread, func: &mut |Box<Thread>, &Thread| -> ()) {
  //let oldi: u32 = unsafe { transmute(old_thread) };
  //let newi: u32 = unsafe { transmute(new_thread) };
  //debug!("old: 0x{:x}; new: 0x{:x}", oldi, newi);
  //old_thread.debug();
  //new_thread.debug();
  
  //let f: u32 = unsafe { transmute(func) };
  //debug!("func: 0x{:x}", f);
  //loop {}
  (*func)(old_thread, new_thread);
  kassert!(false)
}

fn inner_thread_test(arg: uint) {
  debug!("    got int: {}", arg as u32);
}

extern "C" fn test_thread() {
  debug!("in a thread!");
  inner_thread_test(10);
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
    (*s).schedule(test_thread);
    (*s).switch();
  }
}