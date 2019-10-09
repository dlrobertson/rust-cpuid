/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub unsafe fn get_name(bytes: &mut [u8; 12]) -> u32 {
    let value: u32;
    asm!("xorl %eax, %eax
          cpuid
          movl %ebx, (%rdi)
          movl %edx, 0x4(%rdi)
          movl %ecx, 0x8(%rdi)"
          : "=r{eax}"(value) : "{rdi}"(bytes.as_ptr()) : "ebx", "edx", "ecx", "memory");
    value
}

pub unsafe fn get_ext_fn_max() -> u32 {
    let value: u32;
    asm!("movl $$0x80000000, %eax
          cpuid"
         : "=r{eax}"(value) : : "ebx");
    value
}

pub unsafe fn get_brand_string(bytes: &mut [u8]) {
    asm!("movl $$0x80000002, %eax
          movl %eax, %esi
          0:
          cpuid
          movl %eax, (%rdi)
          movl %ebx, 0x4(%rdi)
          movl %ecx, 0x8(%rdi)
          movl %edx, 0xc(%rdi)
          addq $$0x10, %rdi
          addl $$0x1, %esi
          movl %esi, %eax
          cmpl $$0x80000004, %esi
          jle 0b"
         : : "{rdi}"(bytes.as_ptr()) : "rdi", "eax", "esi", "ebx", "ecx", "edx", "memory");
}

pub unsafe fn get_info_bits() -> u64 {
    let mut value: u64 = 0;
    asm!("movl $$0x1, %eax
          cpuid
          movl %edx, (%rdi)
          movl %ecx, 0x4(%rdi)"
         : : "{rdi}"(&mut value as *mut u64) : "edx", "ebx", "ecx", "memory");
    value
}

pub unsafe fn get_ext_feature_bits() -> u64 {
    let mut value: u64 = 0;
    asm!("movl $$0x7, %eax
          cpuid
          movl %ebx, (%rdi)
          movl %ecx, 0x4(%rdi)"
         : : "{rdi}"(&mut value as *mut u64) : "edx", "ebx", "ecx", "memory");
    value
}

pub unsafe fn get_ext_bits() -> u64 {
    let mut value: u64 = 0;
    asm!("movl $$0x80000001, %eax
          cpuid
          movl %edx, (%rdi)
          movl %ecx, 0x4(%rdi)"
         : : "{rdi}"(&mut value as *mut u64) : "edx", "ebx", "ecx", "memory");
    value
}

pub unsafe fn get_stepping_bits() -> u32 {
    let value: u32;
    asm!("movl $$0x1, %eax
          cpuid"
         : "=r{eax}"(value) : : "eax", "edx", "ecx", "ebx");
    value
}
