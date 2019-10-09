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

### Vendor string and setup

```rust
    use cpuid::CPUId;

    fn print_vendor_string() {
        let cpuid = CPUId::new();
        println!("Vendor string: {}", cpuid.vendor());
    }
```

### Supported Features

```rust
    use cpuid::CPUId;
    use cpuid::featinfo::FeatureBit;

    fn supports_sse3() -> bool {
        let cpuid = CPUId::new();
        match cpuid.feature_bits() {
            Ok(features) => features.supports(FeatureBit::Sse3),
            Err(e) => panic!("{:?}", e)
        }
    }
```

### Supported Extensions

```rust
    use cpuid::CPUId;
    use cpuid::featext::ExtensionBit;

    fn supports_syscall() -> bool {
        let cpuid = CPUId::new();
        match cpuid.extension_bits() {
            Ok(extensions) => extensions.supports(ExtensionBit::Syscall),
            Err(e) => panic!("{:?}", e)
        }
    }
```

### Supported Extended Features

```rust
    use cpuid::CPUId;
    use cpuid::featext::FeatureExtensionBit;

    fn supported_avx2() -> bool {
        let cpuid = CPUId::new();
        match cpuid.feature_extension_bits() {
            Ok(feat_ext) => feat_ext.supports(FeatureExtensionBit::Avx2),
            Err(e) => panic!("{:?}", e)
        }
    }
```

### Stepping Model and Family Bits

```rust
    use cpuid::CPUId;

    fn stepping_model_and_family() {
        let cpuid = CPUId::new();
        let cpuinfo = cpuid.smf_bits().unwrap();
        println!("Other Random info\n\tstepping id: {}\tmodel id: {}\t family id: {}",
                 cpuinfo.stepping(), cpuinfo.model(), cpuinfo.family());
    }
```

### 48 Byte Brand String

```rust
    use cpuid::CPUId;

    fn print_brand_string() {
        let cpuid = CPUId::new();
        println!("Brand string: {}", cpuid.brand_string().unwrap());
    }
```
