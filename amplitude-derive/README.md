# Amplitude Event Derive

The `amplitude-derive` crate builds apon the `Event` trait to derive type safe, compile time, implementations of the `Event` trait.

## Structs

```
use amplitude_derive::Event;

#[derive(Event)]
#[ampli(rename = "Button Pressed")]
pub struct ButtonPressed {
    // Button name
    name: String,

    #[ampli(rename = "pressedTimes")]
    pressed_times: u32,
}

```

Derives the trait implementation of `Event` that looks like

```
impl Event for ButtonPressed {
    fn name(&self) -> &str {
        "Button Pressed"
    }

    fn into_event_props(self) -> Vec<Property> {
        vec![
            Property {
                name: "name".to_owned(),
                value: self.name.into(),
            },
            Property {
                name: "pressedTimes".to_owned(),
                value: self.pressed_times.into(),
            }
        ]
    }
}
```

## Enums

```
use amplitude_derive::Event;

#[derive(Event)]
pub enum GameEvents{
    EnemyHit(EnemyHitValue)

    #[ampli(rename = "Player Hit")]
    PlayerHit {
        damage: u32,
    }
}

```

Derives the trait implementation of `Event` that looks like

```
impl Event for GameEvents {
    fn name(&self) -> &str {
       match self {
           Self::EnemyHit(_) => "EnemyHit",
           Self::PlayerHit {
               damage: _
           } => "Player Hit",
       }
    }

    fn into_event_props(self) -> Vec<Property> {
        match self {
            Self::EnemyHit(a) => {
                let mut props = a.into_event_props();
                props
            },
            Self::PlayerHit {
                damage
            } => {
                vec![
                    Property {
                        name: "damage".to_owned(),
                        value: damage.into(),
                    },
                ]
            },
        }
    }
}
```

For enum's the construct is similar however the name is based on the name of the enum variant and if the event props can be combined.

## Attributes

Things can be adjusted by the `#[ampli]` attribute using some of the following properties

- `#[ampli(rename = "Button Pressed")]` can be applied to event names (including invariants) as well as field names

## Restrictions

- Deriving does work on unnamed `structs` because there needs to be a name for each property.
- Enum variants that contained unnamed fields are flattened as long as they implement the `Event` trait
