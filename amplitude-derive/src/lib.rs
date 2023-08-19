mod expand;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

/// The [Event] derive macro will attempt to make your type an Amplitude [Event]
///
/// The intention here is that a type `T` can be an [Event] iff
/// - It is either a `struct` or `enum`
/// - The fields are named
/// - The fields one of the [PropertyValue] types (or convertable too)
///
/// ## Note on Serde attributes
///
/// The reason that we do not use `serde`'s [Serialize] is because we want grabbing the parts for
/// events to be infalliable.
///
/// However some serde properties are planning on being supported i.e. `#[serde(rename = "my_event")]` or `#[ampli(rename = "my_event)]`. It will prioritize the `#[ampli]` attribute.
///
#[proc_macro_derive(Event)]
pub fn event_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::event_derive(input)
        .unwrap_or_else(|e| e.into_compile_error().to_token_stream())
        .into()
}
