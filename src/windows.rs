
use tinystr::TinyAsciiStr;
use windows_sys::Win32::{
    Foundation::SYSTEMTIME,
    System::Time::{GetDynamicTimeZoneInformation, DYNAMIC_TIME_ZONE_INFORMATION},
};
use core::mem::MaybeUninit;

use crate::DynamicTimeZoneError;

#[derive(Debug)]
pub enum DynamicTimeZone {
    StandardTimeZone(DynamicTimeZoneInfo),
    DaylightSavingsTimeZone(DynamicTimeZoneInfo),
}

impl DynamicTimeZone {
    #[inline]
    pub fn get() -> Result<Self, DynamicTimeZoneError> {
        let mut dyn_tz = MaybeUninit::<DYNAMIC_TIME_ZONE_INFORMATION>::zeroed();
        let code = unsafe { GetDynamicTimeZoneInformation(dyn_tz.as_mut_ptr()) };
        DynamicTimeZoneInfo::from_return_code_and_system_info(code, dyn_tz)
    }
}

#[derive(Debug)]
pub struct WinSystemTime {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}

impl From<SYSTEMTIME> for WinSystemTime {
    fn from(value: SYSTEMTIME) -> Self {
        Self {
            wYear: value.wYear,
            wMonth: value.wMonth,
            wDayOfWeek: value.wDayOfWeek,
            wDay: value.wDay,
            wHour: value.wHour,
            wMinute: value.wMinute,
            wSecond: value.wSecond,
            wMilliseconds: value.wMilliseconds,
        }
    }
}

#[derive(Debug)]
pub struct DynamicTimeZoneInfo {
    pub bias: i32,
    pub standard_name: TinyAsciiStr<32>,
    pub standard_date: WinSystemTime,
    pub daylight_name: TinyAsciiStr<32>,
    pub daylight_date: WinSystemTime,
    pub daylight_bias: i32,
    pub tz_key_name: TinyAsciiStr<128>,
    pub dyn_daylight_time_disabled: u8,
}

impl DynamicTimeZoneInfo {
    // TODO: Handle Windows UTF16 better.
    fn try_from_system_info(
        sys_info: DYNAMIC_TIME_ZONE_INFORMATION,
    ) -> Result<Self, DynamicTimeZoneError> {
        Ok(Self {
            bias: sys_info.Bias,
            standard_name: TinyAsciiStr::<32>::try_from_raw_u16(sys_info.StandardName)
                .map_err(|_| DynamicTimeZoneError::IllformedTimeZoneString)?,
            standard_date: WinSystemTime::from(sys_info.StandardDate),
            daylight_name: TinyAsciiStr::<32>::try_from_raw_u16(sys_info.DaylightName)
                .map_err(|_| DynamicTimeZoneError::IllformedTimeZoneString)?,
            daylight_date: WinSystemTime::from(sys_info.DaylightDate),
            daylight_bias: sys_info.DaylightBias,
            tz_key_name: TinyAsciiStr::<128>::try_from_raw_u16(sys_info.TimeZoneKeyName)
                .map_err(|_| DynamicTimeZoneError::IllformedTimeZoneString)?,
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
                DynamicTimeZoneInfo::try_from_system_info(unsafe {info.assume_init() })?,
            )),
            // Safety: Return code from Windows was successful.
            2 => Ok(DynamicTimeZone::DaylightSavingsTimeZone(
                DynamicTimeZoneInfo::try_from_system_info(unsafe {info.assume_init() })?,
            )),
            _ => Err(DynamicTimeZoneError::InvalidReturnCode),
        }
    }
}
