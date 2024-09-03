//! Proof-of-Concept crate for fetching local IANA time zone from windows.

pub mod types;
mod windows_zones;

#[cfg(target_os = "windows")]
mod windows;

use std::{fs::File, io::BufReader};

use types::DynamicTimeZone;
use windows_zones::CldrData;

#[derive(Debug)]
pub enum DynamicTimeZoneError {
    TimeZoneUnknown,
    InvalidReturnCode,
    IllformedTimeZoneString,
    FileReadError,
    DeserializeDataError,
    ParsingError(tinystr::ParseError),
    SyscallErrorCode(u32),
}

#[cfg(target_os = "windows")]
pub fn get_win_time_zone() -> Result<DynamicTimeZone, DynamicTimeZoneError> {
    DynamicTimeZone::get()
}

#[cfg(target_os = "windows")]
pub fn get_iana_time_zone() -> Result<String, DynamicTimeZoneError> {
    use windows::get_geoname;

    let local = DynamicTimeZone::get()?;
    let geoname = get_geoname()?;

    match local {
        DynamicTimeZone::DaylightSavingsTimeZone(zoneinfo) => {
            map_win_tz_to_iana_tz(zoneinfo.tz_key_name.as_str(), Some(geoname.as_str()))
        }
        DynamicTimeZone::StandardTimeZone(zoneinfo) => {
            map_win_tz_to_iana_tz(zoneinfo.tz_key_name.as_str(), Some(geoname.as_str()))
        }
    }
}

// TODO: This is not good. Optimize.
pub fn map_win_tz_to_iana_tz(
    target: &str,
    territory: Option<&str>,
) -> Result<String, DynamicTimeZoneError> {
    let zones_mapping = get_cldr_data()?.supplemental.win_zones.map;
    let target_territory = territory.unwrap_or("001");

    for zone in zones_mapping {
        // Yuck, better structs may help here.
        if zone.map_zone.windows_id == target && zone.map_zone.territory == target_territory {
            return Ok(zone.map_zone.iana_id);
        }
    }

    Err(DynamicTimeZoneError::TimeZoneUnknown)
}

// TODO: Possibly preprocess these in general
fn get_cldr_data() -> Result<CldrData, DynamicTimeZoneError> {
    const WIN_ZONES_PATH: &str = "./cldr-data/cldr-core/supplemental/windowsZones.json";

    let reader = BufReader::new(
        File::open(WIN_ZONES_PATH).map_err(|_| DynamicTimeZoneError::FileReadError)?,
    );
    serde_json::from_reader(reader).map_err(|_| DynamicTimeZoneError::DeserializeDataError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_test_runner() {
        let DynamicTimeZone::DaylightSavingsTimeZone(tz) = get_win_time_zone().unwrap() else {
            panic!()
        };
        println!("{:?}", tz.tz_key_name.as_str());
        println!("{:?}", tz.tz_key_name);
        println!("{tz:?}");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn get_iana_test_runner() {
        let result = get_iana_time_zone().unwrap();
        println!("{result}");
    }

    #[test]
    fn default_mapping_tests() {
        let win_tz = "Afghanistan Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "Asia/Kabul");

        let win_tz = "Eastern Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "America/New_York");

        let win_tz = "Hawaiian Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "Pacific/Honolulu");

        let win_tz = "W. Australia Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "Australia/Perth");

        let win_tz = "Central Europe Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "Europe/Budapest");

        let win_tz = "GMT Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, None).unwrap();
        assert_eq!(&result, "Europe/London");
    }

    #[test]
    fn regional_mapping() {
        let win_tz = "Central Europe Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, Some("CZ")).unwrap();
        assert_eq!(&result, "Europe/Prague");

        let win_tz = "GMT Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, Some("IE")).unwrap();
        assert_eq!(&result, "Europe/Dublin");

        let win_tz = "GMT Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, Some("IE")).unwrap();
        assert_eq!(&result, "Europe/Dublin");

        let win_tz = "Eastern Standard Time";
        let result = map_win_tz_to_iana_tz(&win_tz, Some("BS")).unwrap();
        assert_eq!(&result, "America/Nassau");
    }
}
