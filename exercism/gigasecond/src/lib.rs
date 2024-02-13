use time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECONDS: i64 = 1_000_000_000;
    start + Duration::seconds(GIGASECONDS)
}
