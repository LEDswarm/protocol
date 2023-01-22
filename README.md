# swarm_protocol

[`swarm_protocol`](https://github.com/LEDswarm/protocol) is a compact binary format for swarm messages.

This crate is a support library which is used for communication by server and microcontroller clients, providing Rust methods to encode and decode messages. It does not have any dependencies, so it runs easily in the `no_std` environment of our ESP32 controller modules.