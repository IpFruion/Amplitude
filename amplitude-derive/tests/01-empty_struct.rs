use amplitude::Event;
use amplitude_derive::Event;

#[derive(Event)]
pub struct MyEvent;

fn main() {
    let e = MyEvent;

    assert_eq!(e.name(), "MyEvent");
    let props = e.into_event_props();
    assert_eq!(props.len(), 0);
}
