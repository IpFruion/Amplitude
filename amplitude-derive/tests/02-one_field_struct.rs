use amplitude::{
    event::{Number, PropertyValue},
    Event,
};
use amplitude_derive::Event;

#[derive(Event)]
pub struct MyEvent {
    my_prop: u8,
}

fn main() {
    let e = MyEvent { my_prop: 23 };

    assert_eq!(e.name(), "MyEvent");
    let mut props = e.into_event_props();
    assert_eq!(props.len(), 1);
    let p = props.pop().unwrap();
    assert_eq!(p.name, "my_prop");

    assert_eq!(p.value, PropertyValue::Number(Number::Integer(23)))
}
