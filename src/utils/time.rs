use time::{macros::format_description, OffsetDateTime, UtcOffset, UtcDateTime};

use anyhow::Result;
use anyhow::Context;
use sqlx::types::chrono::NaiveDateTime;

pub(crate) fn now_string() -> Result<String> {
    Ok(current_local_time()
        .format(&format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .context("failed to format current local timestamp")?)
}

pub(crate) fn current_local_time() -> OffsetDateTime {
    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    OffsetDateTime::now_utc().to_offset(local_offset)
}

pub(crate) fn current_native_time() -> NaiveDateTime {
    let local_time = current_local_time();
    NaiveDateTime::from_timestamp_opt(local_time.unix_timestamp(), 0).unwrap()
}

pub(crate) fn from_native_to_offset_time(naive: NaiveDateTime) -> OffsetDateTime {
    let utc_time = naive.and_utc();
    OffsetDateTime::from_unix_timestamp(utc_time.timestamp()).unwrap()
}

pub(crate) fn generate_unique_token(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        OffsetDateTime::now_utc().unix_timestamp_nanos()
    )
}