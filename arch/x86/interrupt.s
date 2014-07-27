.section .text
.global lgdt
.global test
.global no_op
.global interrupt

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

no_op:
  iret
