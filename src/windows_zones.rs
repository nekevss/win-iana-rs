// TODO: Handle serde of windowsZones.json
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CldrData {
    pub(crate) supplemental: SupplementalData,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SupplementalData {
    pub(crate) version: VersionData,
    #[serde(rename = "windowsZones")]
    pub(crate) win_zones: WindowsZones,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct VersionData{
    #[serde(rename = "_unicodeVersion")]
    pub(crate) unicode_version: String,
    #[serde(rename = "_cldrVersion")]
    pub(crate) cldr_version: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WindowsZones {
    #[serde(rename = "mapTimezones")]
    pub(crate) map: Vec<MapZoneContainer>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MapZoneContainer {
    #[serde(rename = "mapZone")]
    pub(crate) map_zone: MapZone,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MapZone {
    #[serde(rename = "_other")]
    pub(crate) windows_id: String,
    #[serde(rename = "_type")]
    pub(crate) iana_id: String,
    #[serde(rename = "_territory")]
    pub(crate) territory: String,
}
