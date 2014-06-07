.section .text
.global lidt
.global lgdt
.global enable_interrupts
.global disable_interrupts
.global test
.global interrupt

test:
  pusha
  call callback
  popa
  iret
  
# args: (pointer to gltd)
lidt:
   mov 4(%esp), %eax
   lidt (%eax)
   ret
   
# args: (pointer to gtd)
lgdt:
   mov 4(%esp), %eax
   lgdt (%eax)
   ret
   
enable_interrupts:
  sti
  ret
  
disable_interrupts:
  cli
  ret

# u8 -> ()
interrupt:
  int $1
  ret
  
