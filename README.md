# Amplitude SDK

This is the unofficial [Amplitude](https://amplitude.com/) SDK for Rust.

The three pieces of this SDK are

- [`amplitude`](https://github.com/IpFruion/Amplitude/tree/main/amplitude/README.md) crate which has the underlying `Event` trait and user `EventOptions` (user properties)
- [`amplitude-derive`](https://github.com/IpFruion/Amplitude/tree/main/amplitude-derive/README.md) crate that implements the derive for `Event`
  - [Structs](#structs)
  - [Enums](#enums)
- [`tracing-amplitude`](https://github.com/IpFruion/Amplitude/tree/main/tracing-amplitude/README.md) crate provides an interface with the [tracing](https://docs.rs/tracing/latest/tracing/) and [tracing_subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) crates to `identify` and `track` amplitude events.
