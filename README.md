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