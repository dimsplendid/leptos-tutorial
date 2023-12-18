# README

## Set up

1. rust
    ```bash
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown

    ```
2. trunk
    ```bash
    cargo install trunk

    ```

## Run

```bash
trunk serve --open
```

> Progress: [Dynamic Attributes](https://book.leptos.dev/view/02_dynamic_attributes.html)