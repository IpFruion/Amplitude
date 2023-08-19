# Amplitude SDK

This is the unofficial [Amplitude](https://amplitude.com/) SDK for Rust.

The three pieces of this SDK are

- `amplitude` crate which has the underlying `Event` trait and user `EventOptions` (user properties)
- `amplitude-derive` crate that implements the derive for `Event`
- `tracing-amplitude` crate provides an interface with the [tracing](https://docs.rs/tracing/latest/tracing/) and [tracing_subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) crates to `identify` and `track` amplitude events.

## Amplitude Event Derive

```
use amplitude_derive::Event;

#[derive(Event)]
pub struct ButtonPressed {
    // Button name
    name: String,
}

```

Derives the trait implementation of `Event` as such

```
impl Event for ButtonPressed {
    fn name(&self) -> &str {
        "ButtonPressed"
    }

    fn into_event_props(self) -> Vec<Property> {
        vec![
            Property {
                name: "name".to_owned(),
                value: self.name.into(),
            }
        ]
    }
}
```

For enum's the construct is similar however the name is based on the name of the enum variant and if the event props can be combined.

#### WIP attributes

Things can be adjusted by the `#[ampli]` attribute using some of the following properties

- `#[ampli(rename = "Button Pressed")]` can be applied to event names as well as field names

## WIP Tracing Amplitude

In the `tracing-amplitude` crate it provides a couple of main functionality

- `AmplitudeLayer` constructs a tracing `Layer` that will cache amplitude event requests and flush them periodically
- `track!` tracks Amplitude events in events thrown by the `tracing` crate
- `identify!` tracks Amplitude user events in events thrown by the `tracing` crate
