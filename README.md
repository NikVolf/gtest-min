# gtest-min
Minimal example for `cargo gtest`.
Try it by running:
`cargo gtest`
or
`cargo gtest --release`

(Install `cargo-gtest` via `cargo install --git https://github.com/NikVolf/onchain-tests`)

## What is minimal

- Dependencies on test-runtime:
```toml
gear-test-codegen = { git = "https://github.com/nikvolf/onchain-tests" }
gear-test-runtime = { git = "https://github.com/nikvolf/onchain-tests" }
```

- Build dependency (improved version of `gear-wasm-builder`):
```toml
gear-ext-builder = { git = "https://github.com/nikvolf/onchain-tests" }
```

## Using
As you can see in `src/tests.rs`, tests are declared with simple decorator!

```rust
#[gear_test_codegen::test]
async fn good(context: &gear_test_runtime::SessionData) {
    let this = create_this(&context.testee()).await;

    let result: Vec<u8> = msg::send_bytes_for_reply(this, b"PING", 0, 0)
        .expect("failed to send")
        .await
        .expect("Program to handle simple PING!!1");

    assert_eq!(result, b"PONG")
}
```
