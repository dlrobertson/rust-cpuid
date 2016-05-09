/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


use std::os::raw::c_char;

pub type CPUIdResult<T> = Result<T, CPUIdErr>;

#[derive(Debug)]
pub enum CPUIdErr {
    NotImplemented(String),
    OutOfRange(u32, u32),
}

#[link(name="cpuid", kind="static")]
extern {
    pub fn cpuid_get_name(bytes: *mut c_char) -> u32;
    pub fn cpuid_get_ext_fn_max() -> u32;
    pub fn cpuid_get_info_bits(x: *mut u64) -> ();
    pub fn cpuid_get_stepping_bits(stepping_id: *mut u8,
                                   model_id: *mut u8,
                                   family_id: *mut u8) -> ();
    pub fn cpuid_get_ext_bits(x: *mut u64) -> ();
    pub fn cpuid_get_ext_feature_bits(x: *mut u64) -> ();
}

