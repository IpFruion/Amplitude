use amplitude::Event;
use amplitude_derive::Event;

#[derive(Event)]
pub enum MyEvents {
    ButtonPressed,
}

fn main() {
    let e = MyEvents::ButtonPressed;

    assert_eq!(e.name(), "ButtonPressed");
    let props = e.into_event_props();
    assert_eq!(props.len(), 0);
}
