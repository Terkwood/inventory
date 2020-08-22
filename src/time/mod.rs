use crate::model::UtcMillis;
use chrono::prelude::*;

pub fn same_date(epoch: UtcMillis, given: Date<FixedOffset>, offset: FixedOffset) -> bool {
    offset.timestamp_millis(epoch.0 as i64).date() == given
}

pub fn js_utc_now() -> UtcMillis {
    UtcMillis(js_sys::Date::now() as u64)
}

/// computes a western-biased FixedOffset using the JS runtime
pub fn js_local_offset() -> FixedOffset {
    FixedOffset::west(js_local_offset_seconds())
}

pub fn _js_local_date(epoch_millis_utc: u64) -> Date<FixedOffset> {
    _js_local_datetime(epoch_millis_utc).date()
}

/// Note the bias to western timezone.
/// If this software were open-sourced, we'd want to make
/// this usable for individuals in the eastern hemisphere.
fn _js_local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
    let offset = FixedOffset::west(js_local_offset_seconds());
    Utc.timestamp_millis(epoch_millis_utc as i64)
        .with_timezone(&offset)
}

const JS_CHRONO_OFFSET_COEFF: i32 = 60;
fn js_local_offset_seconds() -> i32 {
    js_sys::Date::new_0().get_timezone_offset() as i32 * JS_CHRONO_OFFSET_COEFF
}
