use serde::Serialize;

/// Amplitude Event Property
///
/// Contains the name of the property and the value
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

/// Amplitude Number (either a floating point or integer)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Number {
    Float(f64),
    Integer(i64),
}

/// Amplitude Event Property values based on the options set on the site.
///
/// For more information see [website](https://help.amplitude.com/hc/en-us/articles/17050314884635-Set-or-change-a-property-s-data-type)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(Number),
    Boolean(bool),
    Array(Vec<PropertyValue>),
    Enum(Vec<String>),
}

macro_rules! from_integer {
    ($name:ty) => {
        impl From<$name> for PropertyValue {
            fn from(value: $name) -> Self {
                Self::Number(Number::Integer(i64::from(value)))
            }
        }
    };
}

from_integer!(u8);
from_integer!(u16);
from_integer!(u32);
from_integer!(i8);
from_integer!(i16);
from_integer!(i32);
from_integer!(i64);

macro_rules! from_float {
    ($name:ty) => {
        impl From<$name> for PropertyValue {
            fn from(value: $name) -> Self {
                Self::Number(Number::Float(f64::from(value)))
            }
        }
    };
}

from_float!(f32);
from_float!(f64);

macro_rules! from_str {
    ($name:ty) => {
        impl From<$name> for PropertyValue {
            fn from(value: $name) -> Self {
                Self::String(String::from(value))
            }
        }
    };
}

from_str!(String);
from_str!(&str);
