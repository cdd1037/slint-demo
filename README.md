# slint-demo

A minimal Slint + Rust demo for Windows x64, tuned for a small Release binary.

## Stack

- Rust
- Slint `~1.17`
- Winit backend
- FemtoVG renderer
- Windows target: `x86_64-pc-windows-msvc`

The project disables Slint's default features and enables only the standard library, compatibility feature, Winit backend, and FemtoVG renderer. The Release profile uses `opt-level = "z"`, LTO, one codegen unit, `panic = "abort"`, and symbol stripping.

## Build locally

Install the MSVC Rust target, then build:

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

Output:

```text
target\x86_64-pc-windows-msvc\release\slint-demo.exe
```

## GitHub Actions

Every push and pull request builds the Windows x64 Release executable. You can also run the workflow manually from the Actions tab.

The workflow:

1. Builds `slint-demo.exe` on `windows-latest`.
2. Prints the exact EXE byte size and MiB size to the job summary.
3. Creates `slint-demo-windows-x64.zip`.
4. Uploads both the EXE and ZIP as the `slint-demo-windows-x64` artifact.
