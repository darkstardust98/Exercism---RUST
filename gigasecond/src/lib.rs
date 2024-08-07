use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //let start_date = start.assume_utc();
    let duration = Duration::seconds(1_000_000_000);
    let future_date = start + duration;
    future_date
    //println!("{}", future_date);
    //println!("{}", future_date.to_offset(UtcOffset::UTC).date().midnight());
   // future_date.to_offset(UtcOffset::UTC).date().midnight()
}
