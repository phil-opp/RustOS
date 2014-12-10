.global switch_and_save

# switch_and_save(old_thread: &mut Thread, new_thread: &Thread, transferred_info: &[u8], save_to_new_thread: uint);
switch_and_save:
  movl 16(%esp), %eax # should we save?
  cmp $0, %eax
  je switch # or should we just switch?

  # we first want to get the new thread's esp to pass 3 arguments to it:
  movl 8(%esp), %ebx
  movl 28(%ebx), %eax # eax is new thread's esp
  subl $16, %eax # make space for 3 args and rip
  movl %eax, 28(%ebx) # tell the new esp about it...

  movl 12(%esp), %ebx # transfer switch_and_save's 3rd arg
  movl %ebx, 12(%eax) # transfer complete
  
  movl 8(%esp), %ebx # transfer switch_and_save's 2nd arg
  movl %ebx, 8(%eax) # transfer complete

  movl 4(%esp), %ebx # transfer switch_and_save's 1st arg
  movl %ebx, 4(%eax) # transfer complete
    
  movl 4(%esp), %eax # eax is first arg of new thread (i.e., old_thread)
  
  # start thread save ===>  
  movl %ebx, 4(%eax) # TODO(ryan): actually need to save these regs? eax, ebx is actually clobbered here
  movl %ecx, 8(%eax)
  movl %edx, 12(%eax)
  movl %ebp, 16(%eax)
  movl %esi, 20(%eax)
  movl %edi, 24(%eax)  
  movl %esp, 28(%eax)
  movl $out, 32(%eax) # eip; note that we're skipping the 'switch' part when we return
  # <=== end thread save
switch:
  # start context switch ===>
  movl 8(%esp), %eax # new_thread
  
  movl 4(%eax), %ebx
  movl 8(%eax), %ecx
  movl 12(%eax), %edx
  movl 16(%eax), %ebp
  movl 20(%eax), %esi
  movl 24(%eax), %edi
  movl 28(%eax), %esp
  
  movl 32(%eax), %eax #eip
  jmp *%eax # really xfer the eip :D
  # <=== end context switch  
out:
  ret
  