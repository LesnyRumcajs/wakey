# Wakey
[![Build Status](https://travis-ci.com/LesnyRumcajs/wakey.svg?branch=master)](https://travis-ci.com/LesnyRumcajs/wakey)
[![Build Status](http://meritbadge.herokuapp.com/wakey)](https://crates.io/crates/wakey/)
[![docs.rs](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/wakey)

Library for managing Wake-on-LAN packets. It supports:
* creating magic packets,
* broadcasting them via UDP.

# Usage

From string representation of MAC address and using defaults when broadcasting:
```rust
extern crate wakey;

let wol = wakey::WolPacket::from_string("01:02:03:04:05:06", ':');
match wol.send_magic() {
    Ok(_) => println!("Sent the magic packet!"),
    Err(_) => println!("Failed to send the magic packet!")
}
```

Packets can also be constructed with raw bytes and sent from / to custom addresses:
```rust
extern crate wakey;
use std::net::SocketAddr;

let wol = wakey::WolPacket::from_bytes(&vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
let src = SocketAddr::from(([0,0,0,0], 0));
let dst = SocketAddr::from(([255,255,255,255], 9));

wol.send_magic_to(src, dst);
```
