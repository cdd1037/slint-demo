# slint-demo

[![Windows x64 Size Benchmark](https://github.com/cdd1037/slint-demo/actions/workflows/windows-x64.yml/badge.svg)](https://github.com/cdd1037/slint-demo/actions/workflows/windows-x64.yml)

A minimal Slint + Rust demo for Windows x64, tuned and benchmarked for Release binary size.

## Measured Windows x64 size

Measured with Slint `1.17.1`, Rust `1.98.0`, target `x86_64-pc-windows-msvc`, `opt-level = "z"`, fat LTO, one codegen unit, `panic = "abort"`, symbol stripping, and MSVC linker `/OPT:REF /OPT:ICF`.

The UI intentionally uses only built-in `Rectangle`, `Text`, and `TouchArea` elements and does not import `std-widgets.slint`.

| Renderer | EXE bytes | MiB |
| --- | ---: | ---: |
| FemtoVG | 7,066,624 | 6.739 |
| Software | 7,188,992 | 6.856 |

FemtoVG is **122,368 bytes (~0.117 MiB)** smaller than the software-renderer build in this benchmark.

For reference, the earlier FemtoVG build using standard widgets was **7,128,064 bytes (~6.8 MiB)**. The minimal built-in UI plus linker tuning reduced that build by **61,440 bytes (~0.86%)**.

## Stack

- Rust
- Slint `~1.17`
- Winit backend
- Selectable FemtoVG or software renderer
- Windows target: `x86_64-pc-windows-msvc`

Slint default features are disabled. The package enables `std`, `compat-1-2`, `backend-winit`, and exactly one renderer for benchmark builds.

## Build locally

Install the MSVC Rust target first:

```powershell
rustup target add x86_64-pc-windows-msvc
```

FemtoVG:

```powershell
$env:RUSTFLAGS='-C link-arg=/OPT:REF -C link-arg=/OPT:ICF'
cargo build --release --target x86_64-pc-windows-msvc --no-default-features --features renderer-femtovg
```

Software renderer:

```powershell
$env:RUSTFLAGS='-C link-arg=/OPT:REF -C link-arg=/OPT:ICF'
cargo build --release --target x86_64-pc-windows-msvc --no-default-features --features renderer-software
```

Output:

```text
target\x86_64-pc-windows-msvc\release\slint-demo.exe
```

## GitHub Actions

The `Windows x64 Size Benchmark` workflow builds both renderer variants in parallel, reports the exact executable sizes, uploads each executable as a separate artifact, and produces a final comparison table.

Artifacts:

- `slint-demo-femtovg-windows-x64`
- `slint-demo-software-windows-x64`
