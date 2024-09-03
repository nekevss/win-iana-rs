use core::mem::MaybeUninit;
use tinystr::TinyAsciiStr;
use windows_sys::Win32::{
    Foundation::{GetLastError, SYSTEMTIME},
    System::Time::{GetDynamicTimeZoneInformation, DYNAMIC_TIME_ZONE_INFORMATION},
};

use crate::{
    types::{DynamicTimeZone, DynamicTimeZoneInfo, WinSystemTime},
    DynamicTimeZoneError,
};

impl DynamicTimeZone {
    #[inline]
    pub fn get() -> Result<Self, DynamicTimeZoneError> {
        let mut dyn_tz = MaybeUninit::<DYNAMIC_TIME_ZONE_INFORMATION>::zeroed();
        let code = unsafe { GetDynamicTimeZoneInformation(dyn_tz.as_mut_ptr()) };
        DynamicTimeZoneInfo::from_return_code_and_system_info(code, dyn_tz)
    }
}

impl From<SYSTEMTIME> for WinSystemTime {
    fn from(value: SYSTEMTIME) -> Self {
        Self {
            year: value.wYear,
            month: value.wMonth,
            day_of_week: value.wDayOfWeek,
            day: value.wDay,
            hour: value.wHour,
            minute: value.wMinute,
            second: value.wSecond,
            milliseconds: value.wMilliseconds,
        }
    }
}

impl DynamicTimeZoneInfo {
    // TODO: Handle Windows UTF16 better.
    fn try_from_system_info(
        sys_info: DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> Result<Self, DynamicTimeZoneError> {
        Ok(Self {
            bias: sys_info.Bias,
            standard_name: TinyAsciiStr::<32>::try_from_raw_u16(sys_info.StandardName)
                .map_err(|e| DynamicTimeZoneError::ParsingError(e))?,
            standard_date: WinSystemTime::from(sys_info.StandardDate),
            daylight_name: TinyAsciiStr::<32>::try_from_raw_u16(sys_info.DaylightName)
                .map_err(|e| DynamicTimeZoneError::ParsingError(e))?,
            daylight_date: WinSystemTime::from(sys_info.DaylightDate),
            daylight_bias: sys_info.DaylightBias,
            tz_key_name: TinyAsciiStr::<128>::try_from_raw_u16(sys_info.TimeZoneKeyName)
                .map_err(|e| DynamicTimeZoneError::ParsingError(e))?,
            dyn_daylight_time_disabled: sys_info.DynamicDaylightTimeDisabled,
        })
    }
}

impl DynamicTimeZoneInfo {
    #[inline]
    fn from_return_code_and_system_info(
        return_code: u32,
        info: MaybeUninit<DYNAMIC_TIME_ZONE_INFORMATION>,
    ) -> Result<DynamicTimeZone, DynamicTimeZoneError> {
        match return_code {
            0 => Err(DynamicTimeZoneError::TimeZoneUnknown),
            // Safety: Return code from Windows was successful.
            1 => Ok(DynamicTimeZone::StandardTimeZone(
                DynamicTimeZoneInfo::try_from_system_info(unsafe { info.assume_init() })?,
            )),
            // Safety: Return code from Windows was successful.
            2 => Ok(DynamicTimeZone::DaylightSavingsTimeZone(
                DynamicTimeZoneInfo::try_from_system_info(unsafe { info.assume_init() })?,
            )),
            _ => Err(DynamicTimeZoneError::InvalidReturnCode),
        }
    }
}

use windows_sys::Win32::Globalization::GetUserDefaultGeoName;

pub(crate) fn get_geoname() -> Result<TinyAsciiStr<3>, DynamicTimeZoneError> {
    // Geoname MUST be 4 bytes. 3 + Nul => 4
    let mut geoname: [u16; 4] = [0; 4];
    let result = unsafe {GetUserDefaultGeoName(geoname.as_mut_ptr(), 4) };
    if result == 0 {
        return Err(DynamicTimeZoneError::SyscallErrorCode(unsafe { GetLastError()}))
    }
    let trunc_geoname = [geoname[0], geoname[1], geoname[2]];
    TinyAsciiStr::<3>::try_from_raw_u16(trunc_geoname).map_err(|e| DynamicTimeZoneError::ParsingError(e))
}

#[cfg(test)]
mod tests {
    use super::get_geoname;

    #[test]
    fn region_runner() {
        get_geoname().unwrap();
    }
}
