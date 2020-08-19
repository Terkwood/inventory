use chrono::prelude::*;

pub fn same_date_utc(epoch_millis_utc: u64, given: Date<Utc>) -> bool {
    Utc.timestamp_millis(epoch_millis_utc as i64).date() == given
}

pub fn js_utc_now() -> u64 {
    js_sys::Date::now() as u64
}

pub fn _js_local_date(epoch_millis_utc: u64) -> Date<FixedOffset> {
    _js_local_datetime(epoch_millis_utc).date()
}

/// Note the bias to western timezone.
/// If this software were open-sourced, we'd want to make
/// this usable for individuals in the eastern hemisphere.
fn _js_local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
    let offset = FixedOffset::west(_js_local_offset_seconds());
    Utc.timestamp_millis(epoch_millis_utc as i64)
        .with_timezone(&offset)
}

const _JS_CHRONO_OFFSET_COEFF: i32 = 60;
fn _js_local_offset_seconds() -> i32 {
    js_sys::Date::new_0().get_timezone_offset() as i32 * _JS_CHRONO_OFFSET_COEFF
}
