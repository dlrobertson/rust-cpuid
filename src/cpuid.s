.section .text
.global get_name

get_name:
    pushq %rbx
    xorq %rax, %rax
    cpuid
    movl %ebx, 0x0(%rdi)
    movl %edx, 0x4(%rdi)
    movl %ecx, 0x8(%rdi)
    popq %rbx
    ret
