
use tinystr::TinyAsciiStr;

#[derive(Debug)]
pub enum DynamicTimeZone {
    StandardTimeZone(DynamicTimeZoneInfo),
    DaylightSavingsTimeZone(DynamicTimeZoneInfo),
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
