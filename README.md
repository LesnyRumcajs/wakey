# Wakey
[![Rust](https://github.com/LesnyRumcajs/wakey/actions/workflows/rust.yml/badge.svg)](https://github.com/LesnyRumcajs/wakey/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/wakey.svg)](https://crates.io/crates/wakey)
[![docs.rs](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/wakey)

Library for managing Wake-on-LAN packets. It supports:
* creating magic packets,
* broadcasting them via UDP.

# Usage

From string representation of MAC address and using defaults when broadcasting:
```rust
let wol = wakey::WolPacket::from_string("01:02:03:04:05:06", ':');
if wol.send_magic().is_ok() {
    println!("Sent the magic packet!");
} else {
    println!("Failed to send the magic packet!");
}
```

Packets can also be constructed with raw bytes and sent from / to custom addresses:
```rust
use std::net::SocketAddr;

let wol = wakey::WolPacket::from_bytes(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
let src = SocketAddr::from(([0,0,0,0], 0));
let dst = SocketAddr::from(([255,255,255,255], 9));

wol.send_magic_to(src, dst);
```

## Included binary

```
cargo run --bin wakey-wake -m 00:11:22:33:44:55
```
