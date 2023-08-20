use amplitude::{
    event::{Number, Property, PropertyValue},
    Event,
};
use amplitude_derive::Event;

#[derive(Event)]
pub struct ButtonPressed {
    times: u32,
}

#[derive(Event)]
pub enum MyEvents {
    ButtonPressed(ButtonPressed), // Will add up all into event props
    EnemyHit { value: u8 },
}

fn test_enemy_hit() {
    let e = MyEvents::EnemyHit { value: 23 };

    assert_eq!(e.name(), "EnemyHit");
    let props = e.into_event_props();
    assert_eq!(props.len(), 1);
    assert_eq!(
        props[0],
        Property {
            name: "value".to_owned(),
            value: PropertyValue::Number(Number::Integer(23))
        }
    )
}

fn test_button_pressed() {
    let e = MyEvents::ButtonPressed(ButtonPressed { times: 23 });

    assert_eq!(e.name(), "ButtonPressed");
    let props = e.into_event_props();
    assert_eq!(props.len(), 1);
    assert_eq!(
        props[0],
        Property {
            name: "times".to_owned(),
            value: PropertyValue::Number(Number::Integer(23))
        }
    )
}

fn main() {
    test_enemy_hit();
    test_button_pressed();
}
