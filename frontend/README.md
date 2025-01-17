Learning Rust web development stack

## Steps taken:

1. Following [Leptos Getting Started](https://book.leptos.dev/getting_started/index.html) guide
    1. Install [Trunk](https://trunkrs.dev/)
        1. `cargo install trunk`
    2. Add Leptos as dependency
        1. `cargo add leptos --features=csr,nightly`
    3. Use Rust nightly to enable signal features
        1. `rustup override set nightly`
    4. Add wasm32 target
        1. `rustup target add wasm32-unknown-unknown`
    5. Create simple `index.html`
    6. Run server with Trunk
        1. `trunk serve` runs SPA at https://localhost:8080/
2. Set up [dev experience improvements](https://book.leptos.dev/getting_started/leptos_dx.html)
    1. Added `console_error_panic_hook`
    2. Added `leptosfmt.toml` to override default rustfmt on `view` macro

## Notes

### Components

- Definition begins with `#[component]` macro
- Every component returns `impl IntoView`
- The body of the component function is a set-up function that runs once
- `create_signal()` creates a signal, the basic unit of reactive change and state management
    - `let (count, set_count) = create_signal(0)`
- [`view!` macro](https://docs.rs/leptos/latest/leptos/macro.view.html)
    - JSX-like format

## Bugs / Issues I've Run Into

### <u>12/25/2024</u>

- Dependencies conflicting with build target platform
    - `trunk serve` was failing to compile transitive dependencies along path `aws-config -> tokio -> mio`
        - I meant to put `aws-config` in my `/lambdas` sub-workspace but accidentally included it
          in `/frontend/Cargo.toml`
    - `trunk serve` targets `wasm-unknown-unknown`
    - The version of mio used in `/lambdas` was not compatible with `wasm` targets

### <u>7/28/2024</u>

- Blue Screen of Death after some hot-reloading
    - After running `trunk serve -v` and hot-reloading >1 times, Windows crashed
        - Dump logs saved to `../Windows/MEMORY.DMP`
        - Logs viewable w WinDbg
    - My gut feeling is that this is due to Rust being split between C drive and WSL file systems
        - Rustup toolchain was on WSL but project files on C
    - Logged error message shows problem caused by `IMAGE_NAME:  LXCORE.SYS`

- failed to create directory when calling `trunk serve`
    - OS permission error
