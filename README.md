# slint-demo

[![Windows x64 Size Benchmark](https://github.com/cdd1037/slint-demo/actions/workflows/windows-x64.yml/badge.svg)](https://github.com/cdd1037/slint-demo/actions/workflows/windows-x64.yml)

A minimal Slint + Rust demo for Windows x64, tuned and benchmarked for Release binary size and runtime process memory.

## Measured Windows x64 size

Measured with Slint `1.17.1`, Rust `1.98.0`, target `x86_64-pc-windows-msvc`, `opt-level = "z"`, fat LTO, one codegen unit, `panic = "abort"`, symbol stripping, and MSVC linker `/OPT:REF /OPT:ICF`.

The UI intentionally uses only built-in `Rectangle`, `Text`, and `TouchArea` elements and does not import `std-widgets.slint`.

| Renderer | EXE bytes | MiB |
| --- | ---: | ---: |
| FemtoVG | 7,066,624 | 6.739 |
| Software | 7,188,992 | 6.856 |

FemtoVG is **122,368 bytes (~0.117 MiB)** smaller than the software-renderer build in this benchmark.

For reference, the earlier FemtoVG build using standard widgets was **7,128,064 bytes (~6.8 MiB)**. The minimal built-in UI plus linker tuning reduced that build by **61,440 bytes (~0.86%)**.

## Measured idle process memory

GitHub-hosted Windows has no usable hardware OpenGL context for this FemtoVG executable, so the CI comparison runs FemtoVG through Mesa3D `llvmpipe` (CPU software OpenGL). Each app is warmed up for 5 seconds and then sampled 10 times at 1-second intervals on the same runner.

| Renderer | Avg working set | Peak working set | Avg private bytes | Peak private bytes | Avg handles | Avg threads |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| FemtoVG + Mesa llvmpipe | 83.531 MiB | 83.531 MiB | 66.602 MiB | 66.602 MiB | 230 | 15 |
| Slint software renderer | 17.676 MiB | 17.676 MiB | 3.223 MiB | 3.223 MiB | 199 | 8 |

These numbers show that Slint's software renderer is much lighter in this headless CI environment. The FemtoVG number is **not** representative of normal GPU-backed FemtoVG on a Windows desktop because it includes Mesa/llvmpipe CPU rendering overhead. Process Working Set and Private Bytes also do not fully account for GPU-driver or VRAM allocations.

For a representative hardware-accelerated FemtoVG measurement, run the same benchmark on a physical Windows machine or a self-hosted GitHub Actions runner with a real GPU/OpenGL driver.

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

`Windows x64 Size Benchmark` runs on pushes and pull requests, builds both renderer variants, reports exact EXE sizes, and uploads each executable as a separate artifact.

`Windows x64 Memory Benchmark (Mesa CI)` is manual-only. Supply a successful size-benchmark run ID; it downloads both executables, injects Mesa3D llvmpipe for FemtoVG, samples process memory, and uploads the raw `memory.json` result.

Artifacts from the size benchmark:

- `slint-demo-femtovg-windows-x64`
- `slint-demo-software-windows-x64`
