use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Row;
use rusqlite::types::Type;

/// Milliseconds since the Unix epoch, the unit of every timestamp in the catalog.
pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

/// Reads a text column and parses it into a typed value such as a `TagId`.
pub(crate) fn parse<T>(row: &Row<'_>, index: usize) -> rusqlite::Result<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let text: String = row.get(index)?;
    text.parse().map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(index, Type::Text, Box::new(error))
    })
}
