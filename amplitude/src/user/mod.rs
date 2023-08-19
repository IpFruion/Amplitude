use serde::Serialize;

use crate::event::Number;

/// Amplitude User Event Options
///
/// These are defined by the same set in the [Typescript](https://amplitude.github.io/Amplitude-TypeScript/interfaces/_amplitude_analytics_node.Types.EventOptions.html) implementation.
///
/// # Examples
/// ```
/// let options = EventOptions {
///     device_id: Some("someId".to_owned()),
///     ...Default::default(),
/// }
///
/// ```
#[derive(Debug, Default, Serialize)]
pub struct EventOptions {
    pub adid: Option<String>,
    pub android_id: Option<String>,
    pub app_version: Option<String>,
    pub carrier: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub device_brand: Option<String>,
    pub device_id: Option<String>,
    pub device_manufacturer: Option<String>,
    pub device_model: Option<String>,
    pub dma: Option<String>,
    pub event_id: Option<String>,
    /// This one differs from the one in TS because `any` doesn't exist in Rust. Support for the `extra` field might be available in the future.
    /// The recommendation here is to convert the extra fields to a string first
    pub extra: Option<String>,
    pub idfa: Option<String>,
    pub idfv: Option<String>,
    pub ingestion_metadata: Option<IngestionMetadata>,
    pub insert_id: Option<String>,
    pub ip: Option<String>,
    pub language: Option<String>,
    pub library: Option<String>,
    pub location_lat: Option<Number>,
    pub location_lng: Option<Number>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub partner_id: Option<String>,
    pub plan: Option<Plan>,
    pub platform: Option<String>,
    pub price: Option<Number>,
    pub product_id: Option<String>,
    pub quantity: Option<Number>,
    pub region: Option<String>,
    pub revenue: Option<Number>,
    pub revenue_type: Option<String>,
    pub session_id: Option<Number>,
    pub time: Option<Number>,
    pub user_id: Option<String>,
    pub version_name: Option<String>,
}

/// Ingestion Metadata as defined under the [Typescript](https://amplitude.github.io/Amplitude-TypeScript/interfaces/_amplitude_analytics_browser.Types.IngestionMetadata.html) implementation
#[derive(Debug, Default, Serialize)]
pub struct IngestionMetadata {
    pub source_name: Option<String>,
    pub source_version: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct Plan {
    pub branch: Option<String>,
    pub source: Option<String>,
    pub version: Option<String>,
    pub version_id: Option<String>,
}
