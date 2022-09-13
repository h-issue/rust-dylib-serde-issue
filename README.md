### build or run

```
cargo build --features dynamic

## or
cargo run  --features dynamic
```

### issue:

serde dylib works, see this [commit](https://github.com/h-issue/rust-dylib-serde-issue/tree/efee4ba2c7cf6ed1e78e9ddd2b7edb596b6e3653)

but serde_json dylib does not, see this [commit](https://github.com/h-issue/rust-dylib-serde-issue/tree/644eebca125482d22dc4e87595dfc161716338d8)

```
huahouye@adol13u:~/Documents/workspace-rust/dylib-test/rust-dylib-serde-issue$ cargo run --features dynamic
   Compiling rust-dylib-serde-issue v0.1.0 (/home/huahouye/Documents/workspace-rust/dylib-test/rust-dylib-serde-issue)
error: cannot satisfy dependencies so `serde` only shows up once
  |
  = help: having upstream crates all available in one format will likely make this go away

error: could not compile `rust-dylib-serde-issue` due to previous error
```
