use crate::util::FixStr;
use recordkeeper_macros::SaveBin;

/// aka `nn::time::PeriodicBenefitClaimContext`
#[derive(SaveBin, Debug)]
#[size(772)]
pub struct AmiiboTimeData {
    _unk: u32, // 2 if active? <- XC3 field, not part of the sdk
    /// Clock snapshot when the last time check was requested.
    #[loc(0x10)]
    last_request: ClockSnapshot,
    /// Clock snapshot when the amiibo bonus was last received.
    ///
    /// Valid if `last_benefit_received.initial_type != 0`.
    last_benefit_received: ClockSnapshot,
    /// If there is a penalty, this marks its end time.
    #[loc(0x1b0)]
    end_of_penalty: SteadyClockTime,
    unk_2: bool,
}

/// `nn::time::ClockSnapshot`
#[derive(SaveBin, Debug)]
#[size(208)]
struct ClockSnapshot {
    system_time_context: TimeContext,
    network_time_context: TimeContext,

    system_time_posix: i64,
    network_time_posix: i64,

    system_time_calendar: CalendarTime,
    network_time_calendar: CalendarTime,
    system_calendar_info: CalendarAdditionalInfo,
    network_calendar_info: CalendarAdditionalInfo,

    #[loc(0x90)]
    steady_clock_time: SteadyClockTime,

    timezone_name: FixStr<36>,
    #[loc(0xcc)]
    enable_automatic_correction: bool,
    // != 0 => has last benefit received info
    initial_type: u8,
    // 2 u8/bools
}

// https://switchbrew.org/wiki/Glue_services#SystemClockContext
#[derive(SaveBin, Debug)]
#[size(32)]
struct TimeContext {
    system_clock_epoch: i64,
    steady_time: SteadyClockTime,
}

// https://switchbrew.org/wiki/Glue_services#SteadyClockTimePoint
#[derive(SaveBin, Debug)]
#[size(24)]
struct SteadyClockTime {
    /// Seconds since the steady clock epoch
    pub timestamp: u64,
    /// State (seed?/key?), needs to match when comparing
    /// steady clock timestamps
    state: [u8; 16],
}

#[derive(SaveBin, Debug)]
#[size(8)]
struct CalendarTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(SaveBin, Debug)]
struct CalendarAdditionalInfo {
    /// 0-based day of week
    day_of_week: u32,
    /// 0-based day of year
    day_of_year: u32,
    timezone_short_id: FixStr<8>,
    /// Whether daylight savings are currently in effect
    is_dst: bool,
    #[loc(0x14)]
    utc_offset_seconds: i32,
}
