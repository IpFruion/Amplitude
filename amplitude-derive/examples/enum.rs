use amplitude_derive::Event;

#[derive(Event)]
pub struct ButtonPressed {
    times: u32,
}

#[derive(Event)]
pub enum MyEvents {
    ButtonPressed(ButtonPressed),
    EnemyHit { damage: u8, enemy_name: String },
}

fn main() {
    // let e = MyEvents::ButtonPressed;
    //
    // assert_eq!(e.name(), "ButtonPressed");
    // let props = e.into_event_props();
    // assert_eq!(props.len(), 0);
}
