use amplitude::Event;
// use tracing::{callsite, event, Level};
// use tracing_core::LevelFilter;

pub mod filter;

// const AMPLITUDE_EVENT_TARGET: &str = "amplitude_event";

#[macro_export]
macro_rules! track {
    () => {};
}

#[macro_export]
macro_rules! identify {
    () => {};
}

pub fn track<E: Event>(e: E) {
    println!("Name: {}", e.name());
    for prop in e.into_event_props() {
        println!("\t Property:  {}, Value: {:?}", prop.name, prop.value);
    }

    // let c = callsite!(name: AMPLITUDE_EVENT_TARGET, kind: Kind::Event);
    // event!(target: AMPLITUDE_EVENT_TARGET, Level::TRACE, e.name(), eventProps = e.eventProps())
}

#[cfg(test)]
mod tests {}
