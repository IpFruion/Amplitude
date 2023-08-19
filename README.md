# Amplitude SDK

This is the unofficial [Amplitude](https://amplitude.com/) SDK for Rust.

The three pieces of this SDK are

- `amplitude` crate which has the underlying `Event` trait and user `EventOptions` (user properties)
- `amplitude_derive` crate that implements the derive for `Event`
- `tracing-amplitude` crate provides an interface with the [tracing]() and [tracing_subscriber]() crates to `identify` and `track` amplitude events.

It adds the ability to construct Amplitude `Event`s that have associated Amplitude Event properties on them.
