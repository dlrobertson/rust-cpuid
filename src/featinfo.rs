/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use common::{cpuid_get_info_bits, cpuid_get_stepping_bits};

#[repr(u64)]
pub enum FeatureBit {
    Fpu = 0x1,
    Vme = 0x2,
    De = 0x4,
    Pse = 0x8,
    Tsc = 0x10,
    Msr = 0x20,
    Pae = 0x40,
    Mce = 0x80,
    Cx8 = 0x100,
    Apic = 0x200,
    Sep = 0x800,
    Mttr = 0x1000,
    Pge = 0x2000,
    Mca = 0x4000,
    Cmov = 0x8000,
    Pat = 0x10000,
    Pse36 = 0x20000,
    Psn = 0x40000,
    Clfsh = 0x80000,
    Ds = 0x200000,
    Acpi = 0x400000,
    Mmx= 0x800000,
    Fxsr = 0x1000000,
    Sse = 0x2000000,
    Sse2 = 0x4000000,
    Ss = 0x8000000,
    Htt = 0x10000000,
    Tm = 0x20000000,
    Ia64 = 0x40000000,
    Pbe = 0x80000000,
    Sse3 = 0x100000000,
    Pclmulqdq = 0x200000000,
    Dtes64 = 0x400000000,
    Monitor = 0x800000000,
    DesCpl = 0x1000000000,
    Vmx = 0x2000000000,
    Smx = 0x4000000000,
    Est = 0x8000000000,
    Tm2 = 0x10000000000,
    Ssse3 = 0x20000000000,
    CnxtId = 0x40000000000,
    Sdbg = 0x80000000000,
    Fma = 0x100000000000,
    Cx16 = 0x200000000000,
    Xtpr = 0x400000000000,
    Pdcm = 0x800000000000,
    Pcid = 0x2000000000000,
    Dca = 0x4000000000000,
    Sse41 = 0x8000000000000,
    Sse42 = 0x10000000000000,
    X2Apic = 0x20000000000000,
    Movbe = 0x40000000000000,
    Popcnt = 0x80000000000000,
    TscDeadline = 0x100000000000000,
    Aes = 0x200000000000000,
    Xsave = 0x400000000000000,
    OSXsave = 0x800000000000000,
    Avx = 0x1000000000000000,
    F16c = 0x2000000000000000,
    Rdrnd = 0x4000000000000000,
    HyperVisor = 0x8000000000000000
}

pub struct CPUFeatureBits(u64);

impl CPUFeatureBits {
    pub fn new() -> CPUFeatureBits {
        let mut features: u64 = 0;
        unsafe {
            cpuid_get_info_bits(&mut features as *mut u64);
        };
        CPUFeatureBits(features)
    }

    pub fn features(&self) -> u64 {
        self.0
    }

    pub fn supports(&self, feature: FeatureBit) -> bool {
        let bit = feature as u64;
        if bit & self.0 == bit {
            true
        } else {
            false
        }
    }
}

impl Into<u64> for CPUFeatureBits {
    fn into(self) -> u64 {
        self.0
    }
}

pub struct CPUInfo(u8, u8, u8);

impl CPUInfo {
    pub fn new() -> CPUInfo {
        let (mut stepping, mut model, mut family): (u8, u8, u8) = (0, 0, 0);
        unsafe {
            cpuid_get_stepping_bits(&mut stepping as *mut u8,
                                    &mut model as *mut u8,
                                    &mut family as *mut u8);
        };
        CPUInfo(stepping, model, family)
    }

    pub fn stepping(&self) -> u8 {
        self.0
    }

    pub fn model(&self) -> u8 {
        self.1
    }

    pub fn family(&self) -> u8 {
        self.2
    }
}
