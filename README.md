inspired by: https://github.com/letsgetrusty/grpc_example

run server:
```sh
cargo run --bin payments-server
```

if you get error likke this:
```sh
error: failed to parse manifest at `api-rust-grpc/Cargo.toml`

Caused by:
  the cargo feature `different-binary-name` requires a nightly version of Cargo, but this is the `stable` channel
  See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.rust-lang.org/cargo/reference/unstable.html#different-binary-name for more information about using this feature.
```

chek you current rust version :
```sh
rustup toolchain list
# result
stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu
1.61.0-x86_64-unknown-linux-gnu
```

in project dir to change cargo versionrun:
```sh
rustup override set nightly
```

```sh
cargo build
```

```sh
--- stderr
  thread 'main' panicked at '
  failed to execute command: No such file or directory (os error 2)
  is `cmake` not installed?
```

install missing dep:
```sh
sudo apt -y install cmake
```

try again with :
```sh
cargo build
```

Now to create server binary:
```sh
cargo run --bin payments-server
```

Run gRPC client in new terminal 
```sh
cargo run --bin payments-client
```

This will outut:
```js
Response=Response { metadata: MetadataMap { headers: 
{"content-type": "application/grpc", 
"date": "Mon, 27 Jun 2022 14:26:35 GMT", 
"grpc-status": "0"} },
message: BtcPaymentResponse { 
    successful: true, 
    message: "Send 1 BTC to 888888." 
}, extensions: Extensions }
```

and output for server terminal:
```js
We have request: Request { metadata: MetadataMap { headers: 
{"te": "trailers",
"content-type": "application/grpc", 
"user-agent": 
"tonic/0.7.2"} }, 
message: BtcPaymentRequest { 
    from_addr: "123456", 
    to_addr: "888888", 
    amount: 1 
}, extensions: Extensions }
```


## Project statistics
target directory disk size:
```
2 659 items, totalling 1,8 GB
```