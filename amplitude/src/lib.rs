use event::Property;

pub mod event;
pub mod user;

/// A trait that encompass what an Amplitude Event is.
pub trait Event {
    /// The name of the event
    fn name(&self) -> &str;

    /// Converts the type into a list of event properties
    fn into_event_props(self) -> Vec<Property>;

    //TODO: Should we be returning `Rc<str>` and `Rc<[Property]>` instead?
}

#[cfg(test)]
mod tests {

    use crate::{
        event::{Property, PropertyValue},
        Event,
    };

    #[derive(Default)]
    pub struct EnemyHit {
        damage: u32,
        target: String,
    }

    impl Event for EnemyHit {
        fn name(&self) -> &str {
            "Enemy Hit"
        }

        fn into_event_props(self) -> Vec<Property> {
            vec![
                Property {
                    name: "damage".to_owned(),
                    value: self.damage.into(),
                },
                Property {
                    name: "target".to_owned(),
                    value: PropertyValue::String(self.target),
                },
            ]
        }
    }
}
