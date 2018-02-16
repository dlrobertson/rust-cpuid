/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


extern crate cc;

#[cfg(all(target_arch="x86_64"))]
fn main() {
    cc::Build::new().file("src/cpuid.s")
        .static_flag(true)
        .compile("cpuid")
}

#[cfg(not(target_arch="x86_64"))]
fn main() {
    panic!("The target archetecture of this machine is not supported for this crate");
}
