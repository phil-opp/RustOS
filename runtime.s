.global __morestack
.global abort
.global memcmp
.global memcpy
.global malloc
.global free

__morestack:

abort:
    jmp abort

memcmp:
    jmp memcmp

memcpy:
    jmp memcpy

malloc:
    jmp malloc

free:
    jmp free
    