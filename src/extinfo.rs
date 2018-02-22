/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use raw;

#[repr(u64)]
pub enum ExtensionBit {
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
    Syscall = 0x800,
    Mttr = 0x1000,
    Pge = 0x2000,
    Mca = 0x4000,
    Cmov = 0x8000,
    Pat = 0x10000,
    Pse36 = 0x20000,
    Mp = 0x80000,
    Nx = 0x100000,
    MmxExt = 0x400000,
    Mmx = 0x800000,
    Fxsr = 0x1000000,
    FxsrOpt = 0x2000000,
    Pdpe1gb = 0x4000000,
    Rdtscp = 0x8000000,
    Lm = 0x20000000,
    ThreeDNowExt = 0x40000000,
    ThreeDNow = 0x80000000,
    LahfLm = 0x100000000,
    CmpLegacy = 0x200000000,
    Svm = 0x400000000,
    ExtApic = 0x800000000,
    Cr8Legacy = 0x1000000000,
    Abm = 0x2000000000,
    Sse4a = 0x4000000000,
    MisalignSSE = 0x8000000000,
    ThreeDNowPrefetch = 0x10000000000,
    Osvw = 0x20000000000,
    Ibx = 0x40000000000,
    Xop = 0x80000000000,
    Skinit = 0x100000000000,
    Wdt = 0x200000000000,
    Lwp = 0x800000000000,
    Fma4 = 0x1000000000000,
    Tce = 0x2000000000000,
    NodeIDMsr = 0x8000000000000,
    Tbm = 0x10000000000000,
    TopoExt = 0x20000000000000,
    PerfCtrCore = 0x40000000000000,
    PerfctrNb = 0x80000000000000,
    Dbx = 0x200000000000000,
    PerfTsc = 0x400000000000000,
    PcxL2i = 0x800000000000000
}

pub struct CPUExtensionBits(u64);

impl CPUExtensionBits {
    pub fn new() -> CPUExtensionBits {
        let extensions: u64 = unsafe {
            raw::get_ext_bits()
        };
        CPUExtensionBits(extensions)
    }

    pub fn features(&self) -> u64 {
        self.0
    }

    pub fn supports(&self, feature: ExtensionBit) -> bool {
        let bit = feature as u64;
        if bit & self.0 == bit {
            true
        } else {
            false
        }
    }
}

impl Into<u64> for CPUExtensionBits {
    fn into(self) -> u64 {
        self.0
    }
}

