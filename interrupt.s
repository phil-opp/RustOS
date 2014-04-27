.section .text
.global lidt
.global enable_interrupts
.global disable_interrupts

# args: (pointr to gltd)
lidt:
   mov 4(%esp), %eax
   lidt (%eax)
   ret
   
enable_interrupts:
  sti
  ret
  
disable_interrupts:
  cli
  ret
