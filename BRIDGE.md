# Flutter Rust Bridge Integration

1. **Library configuration**
   - `Cargo.toml` now exports a `cdylib` named `mw_atomic_swap`.
   - `src/lib.rs` exposes clean `#[no_mangle] extern "C"` entry points (`mw_atomic_swap_init`, `mw_atomic_swap_offer`, `mw_atomic_swap_accept`, `mw_atomic_swap_finalize`).

2. **Build the native artifact**
   ```powershell
   cd mw-btc-swap
   cargo build --release
   ```
   Copy `target/release/mw_atomic_swap.dll` into your Flutter repo (e.g. `grin_frb_win/windows/runner/`).

3. **Flutter Rust Bridge**
   - In `grin_frb_win`, define your bridge API in Rust/`bridge_definitions.rs`.
   - Run `flutter_rust_bridge_codegen` to generate Dart bindings.
   - Load the DLL via `flutter_rust_bridge` (it will look for `mw_atomic_swap.dll` by default on Windows).

4. **Automation**
   - Create a script (PowerShell/Batch) in `grin_frb_win/build_native.ps1`:
     ```powershell
     cd ..\mw-btc-swap
     cargo build --release
     Copy-Item target\release\mw_atomic_swap.dll ..\grin_frb_win\windows\runner\
     ```
   - Run it before `flutter build windows` so the latest Rust lib is available.
