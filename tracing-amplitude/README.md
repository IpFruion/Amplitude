# WIP Tracing Amplitude

In the `tracing-amplitude` crate it provides a couple of main functionality

- `AmplitudeLayer` constructs a tracing `Layer` that will cache amplitude event requests and flush them periodically
- `track!` tracks Amplitude events in events thrown by the `tracing` crate
- `identify!` tracks Amplitude user events in events thrown by the `tracing` crate
