use recordkeeper_macros::SaveBin;

/// aka `nn::time::PeriodicBenefitClaimContext`
#[derive(SaveBin, Debug)]
#[size(772)]
pub struct AmiiboTimeData {
    _unk: u32, // 2 if active? <- XC3 field, not part of the sdk
    #[loc(0x10)]
    snapshot_1: ClockSnapshot,
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

    system_time_posix: u64,
    network_time_posix: u64,

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

/// Nul-terminated string with fixed storage and maximum length.
///
/// Extra bytes are not guaranteed to be nulls.
#[derive(SaveBin, Debug)]
struct FixStr<const MAX: usize> {
    buf: [u8; MAX],
}

#[derive(SaveBin, Debug)]
#[size(32)]
struct TimeContext {
    steady_time: SteadyClockTime,
    // + 1 unknown u64
}

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
    #[loc(0x4)]
    _unk: u32,
    timezone_short_id: FixStr<8>,
    _unk_2: bool, // DST?
    #[loc(0x14)]
    utc_offset_seconds: i32,
}
