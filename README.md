# rust-cpuid

`rust-cpuid` is (you guessed it!) a simple wrapper around the x86 [`cpuid`](https://en.wikipedia.org/wiki/CPUID) instruction written in [Rust](https://www.rust-lang.org/)

## Install

```
  $ git clone https://github.com/danlrobertson/rust-cpuid.git
  $ cd rust-cpuid
  $ cargo build
  $ cargo test
```

**Note:** The `cpuid` instruction is a x86 instruction. As a result, this crate only supports x86 archetectures (At the moment only x86_64)

## Example

```rust
    use super::CPUId;
    use featinfo::FeatureBit;
    use featext::FeatureExtensionBit;

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
```
