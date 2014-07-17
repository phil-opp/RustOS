.global instruction_pointer
.global stack_pointer
.global set_stack_pointer_and_jump

instruction_pointer:
  popl %eax
  jmp *%eax
  
stack_pointer:
  movl %esp, %eax
  ret
  
#args: stack_pointer, instruction_pointer
set_stack_pointer_and_jump:
  movl 4(%esp), %esp
  jmp *8(%esp)
  

  