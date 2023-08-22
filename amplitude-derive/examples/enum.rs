use amplitude_derive::Event;

#[derive(Event)]
pub struct ButtonPressed {
    times: u32,
}

#[derive(Event)]
pub struct ButtonLinked {
    link: String,
}

#[derive(Event)]
pub enum MyEvents {
    ButtonPressed(ButtonPressed), // Will add up all into event props
    ButtonInteracted(ButtonPressed, ButtonLinked),
    #[ampli(rename = "enemy_hit")]
    EnemyHit {
        value: u8,
        #[ampli(rename = "metadata")]
        meta_data: Option<String>,
    },
}

fn main() {
    // let e = MyEvents::ButtonPressed;
    //
    // assert_eq!(e.name(), "ButtonPressed");
    // let props = e.into_event_props();
    // assert_eq!(props.len(), 0);
}
