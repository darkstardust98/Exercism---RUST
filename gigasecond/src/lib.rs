use time::{Duration, OffsetDateTime, PrimitiveDateTime as DateTime, UtcOffset};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let start_date = start.assume_utc();
    let duration = Duration::seconds(1_000_000_000);
    let future_date = start_date + duration;
    future_date.to_offset(UtcOffset::UTC).date().midnight()
}
