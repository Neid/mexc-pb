# 📦 mexc-pb

Rust data structures and Protobuf bindings for the [MEXC WebSocket
API](https://github.com/mexcdevelop/websocket-proto), generated with
[`prost`](https://crates.io/crates/prost).

This crate provides strongly typed Rust structs and enums derived from
the official `.proto` schemas, so you can easily (de)serialize messages
when building trading bots, market data consumers, or other integrations
with MEXC.

------------------------------------------------------------------------

## 🚀 Features

-   ✅ Rust bindings generated directly from the official [MEXC `.proto`
    definitions](https://github.com/mexcdevelop/websocket-proto)
    (included as a git submodule).\
-   ✅ Zero hand-written message structs --- always in sync with
    upstream `.proto`.\
-   ✅ Uses [`prost`](https://crates.io/crates/prost) for efficient
    Protobuf encoding/decoding.\
-   ✅ Suitable for WebSocket clients, real-time feeds, and automated
    trading tools.

------------------------------------------------------------------------

## 📦 Installation

Add this crate to your project:

``` toml
[dependencies]
mexc-pb = "0.1"
```

------------------------------------------------------------------------

## 🛠 Usage

``` rust
use prost::Message;
use mexc_pb::pb::websocket::PushDataV3ApiWrapper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example binary
    let encoded = b"\n3spot@public.aggre.bookTicker.v3.api.pb@10ms@SOLUSDT\x1a\x07SOLUSDT0\x97\xbf\xd7\xff\x923\xda\x13 \n\x06216.69\x12\x06953.77\x1a\x05216.7\"\x07113.135";
    let book_ticker = PushDataV3ApiWrapper::decode(encoded.as_slice())?;
    println!("Book Ticker: {:?}", book_ticker);
    Ok(())
}
```

------------------------------------------------------------------------

## 🔄 Development

This crate vendors the `.proto` files via a git submodule:

``` bash
git clone https://github.com/your-org/mexc-pb.git
cd mexc-pb
git submodule update --init --recursive
```

Generated code lives under [`src/pb/`](src/pb/) and is produced
automatically at build time by
[`prost-build`](https://crates.io/crates/prost-build).

To rebuild generated bindings:

``` bash
cargo clean
cargo build
```

------------------------------------------------------------------------

## 📜 License

This project is licensed under the [MIT License](LICENSE).\
The `.proto` definitions are from
[mexcdevelop/websocket-proto](https://github.com/mexcdevelop/websocket-proto),
which may be licensed separately.
