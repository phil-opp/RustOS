.global __morestack
.global memcmp
.global memcpy

__morestack:
    jmp __morestack

memcmp:
    jmp memcmp
    