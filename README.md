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
    fn it_works() {
        let cpuinfo = CPUId::new();
        assert_eq!("GenuineIntel", cpuinfo.vendor());
    }
```
