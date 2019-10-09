/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![feature(asm)]
#![no_std]

#[cfg(test)]
extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate std;

mod common;
pub mod extinfo;
pub mod featext;
pub mod featinfo;
mod raw;

use crate::common::{CPUIdErr, CPUIdResult};
use crate::extinfo::CPUExtensionBits;
use crate::featext::CPUFeatureExtensionBits;
use crate::featinfo::{CPUFeatureBits, CPUInfo};
use core::str;

#[derive(Debug)]
pub struct CPUId {
    vendor_str: [u8; 12],
    high_value: u32,
    ext_fn_max: Option<u32>,
}

impl CPUId {
    pub fn new() -> CPUId {
        let mut res: [u8; 12] = [0; 12];
        let high_value = unsafe { raw::get_name(&mut res) };
        let max = if high_value < 1 {
            None
        } else {
            Some(unsafe { raw::get_ext_fn_max() })
        };
        CPUId {
            vendor_str: res,
            high_value: high_value,
            ext_fn_max: max,
        }
    }

    pub fn high_value(&self) -> u32 {
        self.high_value
    }

    pub fn vendor<'a, 'b: 'a>(&'b self) -> Result<&'a str, str::Utf8Error> {
        str::from_utf8(&self.vendor_str)
    }

    pub fn ext_fn_max(&self) -> Option<u32> {
        self.ext_fn_max
    }

    pub fn feature_bits(&self) -> CPUIdResult<CPUFeatureBits> {
        if self.high_value <= 0x1 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUFeatureBits::new())
        }
    }

    pub fn smf_bits(&self) -> CPUIdResult<CPUInfo> {
        if self.high_value <= 0x1 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUInfo::new())
        }
    }

    pub fn extension_bits(&self) -> CPUIdResult<CPUExtensionBits> {
        if self.ext_fn_max.map(|x| x <= 0x80000001).unwrap_or(false) {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUExtensionBits::new())
        }
    }

    pub fn brand_string(&self, buf: &mut [u8]) -> CPUIdResult<()> {
        if self.ext_fn_max.map(|x| x <= 0x80000004).unwrap_or(false) {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else if buf.len() >= 48 {
            unsafe {
                raw::get_brand_string(buf);
            }
            Ok(())
        } else {
            Err(CPUIdErr::BufferTooSmall)
        }
    }

    pub fn feature_extension_bits(&self) -> CPUIdResult<CPUFeatureExtensionBits> {
        if self.high_value < 0x7 {
            Err(CPUIdErr::OutOfRange(self.high_value, 1))
        } else {
            Ok(CPUFeatureExtensionBits::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CPUId;
    use crate::extinfo::ExtensionBit;
    use crate::featext::FeatureExtensionBit;
    use crate::featinfo::FeatureBit;

    #[test]
    fn it_works() {
        let cpuid = CPUId::new();
        println!("Vendor string: {}", cpuid.vendor().unwrap());
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
                let cpuinfo = cpuid.smf_bits().unwrap();
                println!(
                    "Other Random info\n\tstepping id: {}\tmodel id: {}\t family id: {}",
                    cpuinfo.stepping(),
                    cpuinfo.model(),
                    cpuinfo.family()
                );
                let mut buf = [0; 48];
                cpuid.brand_string(&mut buf).unwrap();
                println!("{}", std::str::from_utf8(&buf).unwrap());
            }
            None => {
                println!("Could not obtain feature information");
            }
        }
    }
}
