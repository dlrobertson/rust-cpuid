/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


pub mod common;
pub mod featinfo;
pub mod extinfo;
pub mod featext;

use std::ffi::CString;
use common::{CPUIdErr, CPUIdResult};
use common::{cpuid_get_ext_fn_max, cpuid_get_name};
use featinfo::{CPUFeatureBits, CPUInfo};
use extinfo::{CPUExtensionBits};
use featext::{CPUFeatureExtensionBits};

#[derive(Debug)]
pub struct CPUId {
    vendor_str: String,
    high_value: u32,
    ext_fn_max: Option<u32>
}

impl CPUId {
    pub fn new() -> CPUId {
        let (high_value, vendor_str) = unsafe {
            let res = CString::new("             ").unwrap().into_raw();
            let value = cpuid_get_name(res);
            let vendor_str = String::from_raw_parts(res as *mut u8, 12, 12);
            (value, vendor_str)
        };
        let max = if high_value < 1 {
            None
        } else {
            Some(unsafe { cpuid_get_ext_fn_max() })
        };
        CPUId {
            vendor_str: vendor_str,
            high_value: high_value,
            ext_fn_max: max
        }
    }

    pub fn high_value(&self) -> u32 {
        self.high_value
    }

    pub fn vendor(&self) -> String {
        self.vendor_str.clone()
    }

    pub fn ext_fn_max(&self) -> Option<u32> {
        self.ext_fn_max
    }

    pub fn feature_bits(&self) -> CPUIdResult<CPUFeatureBits> {
        if self.high_value < 1 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUFeatureBits::new())
        }
    }

    pub fn stepping_bits(&self) -> CPUIdResult<CPUInfo> {
        if self.high_value < 1 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUInfo::new())
        }
    }

    pub fn extension_bits(&self) -> CPUIdResult<CPUExtensionBits> {
        if self.high_value < 1 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUExtensionBits::new())
        }
    }

    pub fn feature_extension_bits(&self) -> CPUIdResult<CPUFeatureExtensionBits> {
        if self.high_value < 7 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUFeatureExtensionBits::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CPUId;
    use featinfo::FeatureBit;
    use extinfo::ExtensionBit;
    use featext::FeatureExtensionBit;
    #[test]
    fn it_works() {
        let cpuid = CPUId::new();
        println!("Vendor string: {}", cpuid.vendor());
        match cpuid.ext_fn_max() {
            Some(_) => {
                // CPU Feature Bits
                let features = cpuid.feature_bits().unwrap();
                if features.supports(FeatureBit::Sse3) {
                    println!("CPU Supports SSE3!");
                }
                // CPU Extension Bits
                let extensions = cpuid.extension_bits().unwrap();
                if extensions.supports(ExtensionBit::Syscall) {
                    println!("CPU Supports syscall!");
                }
                // CPU Feature Extension Bits
                let feature_ext = cpuid.feature_extension_bits().unwrap();
                if feature_ext.supports(FeatureExtensionBit::Avx2) {
                    println!("CPU Supports AVX2!");
                }
                // CPU Stepping, Model, and Family Bits
                let cpuinfo = cpuid.stepping_bits().unwrap();
                println!("Other Random info\n\tstepping id: {}\tmodel id: {}\t family id: {}",
                         cpuinfo.stepping(), cpuinfo.model(), cpuinfo.family());
            },
            None => { println!("Could not obtain feature information"); }
        }
    }
}
