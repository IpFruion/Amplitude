use amplitude::{
    event::{Number, PropertyValue},
    Event,
};
use amplitude_derive::Event;

#[derive(Event)]
#[ampli(rename = "my_event")]
pub struct MyEvent {
    #[ampli(rename = "myProp")]
    my_prop: u8,
}

fn main() {
    let e = MyEvent { my_prop: 23 };

    assert_eq!(e.name(), "my_event");
    let mut props = e.into_event_props();
    assert_eq!(props.len(), 1);
    let p = props.pop().unwrap();
    assert_eq!(p.name, "myProp");

    assert_eq!(p.value, PropertyValue::Number(Number::Integer(23)))
}
