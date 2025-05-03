# LibP2P Chat with FFI

A demonstration of using [libp2p](https://github.com/libp2p/rust-libp2p) in Rust with C FFI bindings, showcasing peer-to-peer communication capabilities in a cross-language environment, for [Rust Connect #1 in Istanbul, Turkey 04/05/2025](https://lu.ma/7eznvozi).

## Usage

You will need Rust to use this project. Clone the repository to get started:

```sh
git clone https://github.com/yourusername/rustconnect-libp2p
cd rustconnect-libp2p
```

Then, build the library:

```sh
cargo build --release
```

You can run with:

```sh
cargo run
```

You can type a text to the terminal, and when you press <kbd>ENTER</kbd> it will be published to the network.
To exit the application, you must write `exit` and enter.

### FFI

You need a C compiler (`gcc` / `clang`) for the FFI example. After building the Rust library, go to `ffi` directory and build the C binary:

```sh
cd ffi
make

# or `make again` if you have changes & want to force re-build
```

Now you can run the binary with:

```sh
./build/main

# can enable logs as well:
RUST_LOG=info ./build/main
```

This will listen to messages on the network; to terminate the application simply do <kbd>CTRL+C</kbd>.
How to publish messages is left as an exercise!

> [!NOTE]
>
> The FFI functions are exported via `external` feature, which is enabled by default.

## Documentation

You can view the crate documentation with:

```sh
cargo doc --open --no-deps --document-private-items
```

## License

Licensed under the [MIT License](./LICENSE).
