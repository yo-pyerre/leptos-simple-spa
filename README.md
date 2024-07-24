Learning Rust web development stack

Steps taken:
1. Following [Leptos Getting Started](https://book.leptos.dev/getting_started/index.html) guide
   1. Install [Trunk](https://trunkrs.dev/)
      1. `cargo install trunk`
   2. Add Leptos as dependency
      1. `cargo add leptos --features=csr`
   3. Use Rust nightly to enable signal features
      1. `rustup override set nightly`
   4. Add wasm32 target
      1. `rustup target add wasm32-unknown-unknown`
   5. Create simple `index.html`
