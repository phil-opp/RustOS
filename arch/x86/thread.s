.global instruction_pointer
.global stack_pointer
.global set_pointers_and_jump
.global base_pointer
  
# args: foo
instruction_pointer:
  popl %eax
  jmp *%eax
  
stack_pointer:
  movl %esp, %eax
  addl $4, %eax 
  ret
  
base_pointer:
  movl %ebp, %eax
  ret
  
#args: stack_pointer, base_pointer, instruction_pointer
# we shouldn't clobber eax here as it's used to signal a resume
set_pointers_and_jump:
  movl $0, %eax # this is the return value (NULL) for call to instruction_pointer()
  movl %esp, %ebx
  movl 4(%ebx), %esp
  movl 8(%ebx), %ebp
  jmp *12(%ebx)
  
  