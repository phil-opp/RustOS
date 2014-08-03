.section .text
.global lgdt
.global test
.global no_op
.global interrupt
.global unified_handler
.global register_all_callbacks
.global callback_0 

no_op:
test:
  pusha
  call callback
  popa
  iret
  
# args: (pointer to gtd)
lgdt:
   mov 4(%esp), %eax
   lgdt (%eax)
   ret

# u8 -> ()
interrupt:
  int $2
  ret


.altmacro
  
.macro make_callback num
  callback_\num\():
.endm

.macro make_all_callbacks, num=10
.if \num+1
   make_callback %num 
      #cli
      #jmp loop
      pusha
      pushl $\num
      #call unified_handler
      
      call callback_i
      addl 4, %esp
      popa
      #sti
      iret
  make_all_callbacks \num-1
.endif
.endm
make_all_callbacks

.macro push_callback num
  pushl $callback_\num\()
.endm

loop:
  pushl (0x5)
  jmp loop
# arguments &mut IDT
/*register_all_callbacks_0:
pushl %ebp
movl %esp, %ebp
pushl callback_50
addl 4, %esp
leave
ret
*/

register_all_callbacks:
  pushl %ebp
  movl %esp, %ebp
  
  .macro make_register_all_callbacks, num=10
    .if \num+1
	  push_callback %num # arg3 (fn) to add_entry
	  pushl $\num # arg2 (index) to add_entry
	  movl 8(%ebp), %eax
	  pushl %eax # arg1 (&self) to add_entry
	  call add_entry
	  movl %ebp, %esp
      make_register_all_callbacks \num-1
    .endif
  .endm
  make_register_all_callbacks
    
  leave
  ret
