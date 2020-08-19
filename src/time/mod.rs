pub fn js_utc_now() -> u64 {
    js_sys::Date::now() as u64
}
