# TODO(ryan): rust seems to use libc's implementation of these functions
# and I'm too lazy to strip them out of rust. An alternative might be to 
# just find the .so file for them and add it to the linker (and hope it
# doesn't pull any linux dependencies

.globl EXHAUSTED
.globl __mulodi4
.globl __divdi3
.globl __umoddi3
.globl rust_begin_unwind
.globl __udivdi3
.globl __moddi3
.globl __powisf2
.globl __powidf2
.globl __fixunssfdi
.globl __fixunsdfdi

.global abort


EXHAUSTED:
__mulodi4:
__divdi3:
__umoddi3:
rust_begin_unwind:
__udivdi3:
__moddi3:
__powisf2:
__powidf2:
__fixunssfdi:
__fixunsdfdi:
  call abort
  