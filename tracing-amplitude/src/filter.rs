use tracing::Metadata;
use tracing_subscriber::layer::{Context, Filter};

/// Creates a [tracing_subscriber::layer::Filter] to remove Amplitude Events from [tracing_subscriber::layer::Layer] or [tracing::Subscriber]
pub struct RemoveAmplitudeEvents;

impl<S> Filter<S> for RemoveAmplitudeEvents {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        meta.target().eq("amplitude_events")
    }
}
