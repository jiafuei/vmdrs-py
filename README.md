vmdrs-py
---

Example library to use [vmd-rs](https://github.com/jiafuei/vmd-rs) crate.


Installation
---
```
pip install vmdrs-py
```

Compiling from source
---
Rust compiler is needed since this project is compiled using [Maturin](https://github.com/PyO3/maturin). 
```
pip install maturin
maturin build --release
```

Usage
---
See [vmdpy](https://github.com/vrcarva/vmdpy)

Pre-built Binaries
---

| Target                    | Blas Provider        |
|---------------------------|----------------------|
| aarch64-unknown-linux-gnu | OpenBLAS             |
| x86_64-unknown-linux-gnu  | Intel MKL            |
| x86_64-pc-windows-msvc    | Intel MKL            |
| x86_64-apple-darwin       | Accelerate Framework |
| aarch64-apple-darwin      | Accelerate Framework |
