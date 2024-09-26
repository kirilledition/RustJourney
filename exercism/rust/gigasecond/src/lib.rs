use time::PrimitiveDateTime as DateTime;

const GIGASECOND: i64 = 1e9 as i64;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let seconds_after_timestamp = time::Duration::new(GIGASECOND, 0);
    start + seconds_after_timestamp
}
