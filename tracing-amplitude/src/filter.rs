use tracing::Metadata;
use tracing_subscriber::layer::{Context, Filter};

/// Creates a [Filter] to remove Amplitude Events from [Layer] or [Subscriber]
pub struct RemoveAmplitudeEvents;

impl<S> Filter<S> for RemoveAmplitudeEvents {
    fn enabled(&self, meta: &Metadata<'_>, _cx: &Context<'_, S>) -> bool {
        meta.target().eq("amplitude_events")
    }
}
