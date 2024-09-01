//! Proof-of-Concept crate for fetching local IANA time zone from windows.

mod windows;
pub mod types;

use windows::DynamicTimeZone;

#[derive(Debug)]
pub enum DynamicTimeZoneError {
    TimeZoneUnknown,
    InvalidReturnCode,
    IllformedTimeZoneString,
}


pub fn get_iana_time_zone() -> Result<DynamicTimeZone, DynamicTimeZoneError> {
    DynamicTimeZone::get()
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_test_runner() {
        let DynamicTimeZone::DaylightSavingsTimeZone(tz) = get_iana_time_zone().unwrap() else { panic!() };
        println!("{:?}", tz.tz_key_name.as_str());
        println!("{:?}", tz.tz_key_name);
        println!("{tz:?}");
    }
}
