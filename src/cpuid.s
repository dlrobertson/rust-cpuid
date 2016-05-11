# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


.section .text
.global cpuid_get_name
.global cpuid_get_ext_fn_max
.global cpuid_get_info_bits
.global cpuid_get_stepping_bits
.global cpuid_get_ext_bits
.global cpuid_get_ext_feature_bits
.global cpuid_get_brand_string

cpuid_get_name:
    pushq %rbx
    xorq %rax, %rax
    cpuid
    movl %ebx, 0x0(%rdi)
    movl %edx, 0x4(%rdi)
    movl %ecx, 0x8(%rdi)
    popq %rbx
    ret

cpuid_get_ext_fn_max:
    pushq %rbx
    movq $0x80000000, %rax
    cpuid
    popq %rbx
    ret

cpuid_get_info_bits:
    pushq %rbx
    movq $0x1, %rax
    cpuid
    movl %edx, (%rdi)
    movl %ecx, 0x4(%rdi)
    popq %rbx
    ret

cpuid_get_stepping_bits:
    pushq %rbx
    pushq %rbp
    movq %rsp, %rbp
    movq %rdx, (%rsp)
    movq $0x1, %rax
    cpuid
    movq %rax, %rbx
    andq $0xf, %rbx          # get stepping bits
    movq %rbx, (%rdi)        # save to first arg
    movq %rax, %rbx
    movq %rax, %rcx
    andq $0xf0, %rbx         # get model bits
    shrq $0x4, %rbx
    andq $0xf0000, %rcx      # get ext model bits
    shrq $0x8, %rcx
    orq  %rbx, %rcx
    movq %rcx, (%rsi)        # save to second arg
    movq %rax, %rbx
    movq %rax, %rcx
    andq $0xf00, %rbx        # get family bits
    shrq $0x8, %rbx
    andq $0xf00000, %rcx     # get ext family bits
    shrq $0x10, %rcx
    orq  %rbx, %rcx
    movq (%rsp), %rdx
    movq %rcx, (%rdx)        # save to third arg
    popq %rbp
    popq %rbx
    ret

cpuid_get_ext_feature_bits:
    pushq %rbx
    xorq %rcx, %rcx
    movq $0x7, %rax
    cpuid
    movl %ebx, (%rdi)
    movl %ecx, 0x4(%rdi)
    popq %rbx
    ret

cpuid_get_ext_bits:
    pushq %rbx
    movq $0x80000001, %rax
    cpuid
    movl %edx, (%rdi)
    movl %ecx, 0x4(%rdi)
    popq %rbx
    ret

cpuid_get_brand_string:
    pushq %rbx
    movq $0x80000002, %rax
    movq %rax, %rsi
.L0:
    cpuid
    movl %eax, (%rdi)
    movl %ebx, 0x4(%rdi)
    movl %ecx, 0x8(%rdi)
    movl %edx, 0xc(%rdi)
    addq $0x10, %rdi
    addq $0x1, %rsi
    movq %rsi, %rax
    cmpl $0x80000004, %esi
    jle .L0
    popq %rbx
    ret
