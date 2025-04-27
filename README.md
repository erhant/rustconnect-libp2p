# LibP2P Chat with FFI

A demonstration of using [libp2p](https://github.com/libp2p/rust-libp2p) in Rust with C FFI bindings, showcasing peer-to-peer communication capabilities in a cross-language environment, for [Rust Connect #1 in Istanbul, Turkey 04/05/2025](https://lu.ma/7eznvozi).

- **Peer-to-Peer Chat**: Built on libp2p, enabling direct communication between peers
- **FFI Support**: Complete C bindings for the Rust libp2p implementation
- **Cross-Platform**: Works on Unix-based systems
- **Simple API**: Straightforward C interface for integration

## Installation

### Prerequisites

- Rust (latest stable)
- C compiler (gcc/clang) optionally for the FFI example

### Building

Clone the repository:

```sh
git clone https://github.com/yourusername/rustconnect-libp2p
cd rustconnect-libp2p
```

Build the Rust library:

```sh
cargo build --release
```

Afterwards, you can optionally build the C example binary:

```sh
cd ffi
make
```

## Usage

You can run with:

```sh
cargo run
```

> [!TIP]
>
> You can run without cloning at all too:
>
> ```sh
> cargo install https://github.com/yourusername/rustconnect-libp2p
> rustconnect-libp2p
> ```

You can run the C example (after building) with:

```sh
cd ffi
./build/main
```

## License

Licensed under the [MIT License](./LICENSE).
