# swarm_protocol

[`swarm_protocol`](https://github.com/LEDswarm/protocol) is a compact binary format for swarm messages.

This crate is a support library which is used for communication by the server and its microcontroller clients, providing Rust methods to encode and decode messages. It does not have any dependencies, so it runs easily in the `no_std` environment of our ESP32 controller modules.

In the future, our ESP32 modules will also feature a server mode, so we can play without having a base station present. At this stage, however, it's not really a priority for me since I'm getting the basic stuff running.