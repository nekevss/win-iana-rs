use tinystr::TinyAsciiStr;

#[derive(Debug)]
pub enum DynamicTimeZone {
    StandardTimeZone(DynamicTimeZoneInfo),
    DaylightSavingsTimeZone(DynamicTimeZoneInfo),
}

#[derive(Debug)]
pub struct WinSystemTime {
    pub year: u16,
    pub month: u16,
    pub day_of_week: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub milliseconds: u16,
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
