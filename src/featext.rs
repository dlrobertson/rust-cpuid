/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use raw;

#[repr(u64)]
pub enum FeatureExtensionBit {
    Fsgsbase = 0x1,
    Sgx = 0x4,
    Bmi1 = 0x8,
    Hle = 0x10,
    Avx2 = 0x20,
    Smep = 0x80,
    Bmi2 = 0x100,
    Erms = 0x200,
    Invpcid = 0x400,
    Rtm = 0x800,
    Pqm = 0x1000,
    Mpx = 0x4000,
    Pqe = 0x8000,
    Avx512f = 0x10000,
    Avx512dq = 0x20000,
    Rdseed = 0x40000,
    Adx = 0x80000,
    Smap = 0x100000,
    Avx512ifma = 0x200000,
    Pcommit = 0x400000,
    ClFlushOpt = 0x800000,
    Clwb = 0x1000000,
    Avx512pf = 0x4000000,
    Avx512er = 0x8000000,
    Avx512cd = 0x10000000,
    Sha = 0x20000000,
    Avx512bw = 0x40000000,
    Avx512vi = 0x80000000,
    Prefetchwt1 = 0x100000000,
    Avx512vbmi = 0x200000000
}

pub struct CPUFeatureExtensionBits(u64);

impl CPUFeatureExtensionBits {
    pub fn new() -> CPUFeatureExtensionBits {
        let extensions: u64 = unsafe {
            raw::get_ext_feature_bits()
        };
        CPUFeatureExtensionBits(extensions)
    }

    pub fn features(&self) -> u64 {
        self.0
    }

    pub fn supports(&self, feature: FeatureExtensionBit) -> bool {
        let bit = feature as u64;
        if bit & self.0 == bit {
            true
        } else {
            false
        }
    }
}

impl Into<u64> for CPUFeatureExtensionBits {
    fn into(self) -> u64 {
        self.0
    }
}
