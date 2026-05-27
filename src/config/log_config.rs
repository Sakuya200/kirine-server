use std::{fmt as stdfmt, fs, io, io::IsTerminal, path::PathBuf, sync::OnceLock, thread};

use anyhow::{Context, Result};
use time::{OffsetDateTime, UtcOffset, format_description::FormatItem, macros::format_description};
use tracing::{info, Subscriber};
use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    EnvFilter,
    fmt::{
        self, FmtContext,
        format::{FormatEvent, FormatFields, Writer},
        time::FormatTime,
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
};
use crate::utils::path::resolve_log_dir;

static LOG_GUARD: OnceLock<WorkerGuard> = OnceLock::new();
static LOCAL_TIME_OFFSET: OnceLock<UtcOffset> = OnceLock::new();

const LOG_FILE_PREFIX: &str = "kirine-server.log";
const CLIENT_TIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

pub fn init_log(configured_log_dir: &PathBuf) {
    let log_dir = resolve_log_dir(configured_log_dir);
    if let Err(e) = log_dir {
        panic!("failed to resolve log directory {:?}: {}", configured_log_dir, e);
    }

    let log_dir = log_dir.unwrap();
    let file_appender = rolling::daily(&log_dir, LOG_FILE_PREFIX);
    let (writer, guard) = non_blocking(file_appender);
    let _ = LOG_GUARD.set(guard);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_directives()));

    let console_layer = fmt::layer()
        .with_ansi(io::stderr().is_terminal())
        .with_writer(io::stderr)
        .with_thread_names(false)
        .event_format(ClientLogFormatter::default());

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(writer)
        .with_thread_names(false)
        .event_format(ClientLogFormatter::default());

    let result = tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .try_init();
    if let Err(err) = result {
        panic!("failed to initialize log, failed to register tracing subscriber: {}", err);
    }

    info!(log_dir = %log_dir.display(), "logging initialized");
}

fn default_directives() -> &'static str {
    if cfg!(debug_assertions) {
        "info,kirine_server_lib=debug,kirine_server=debug"
    } else {
        "info"
    }
}

#[derive(Default)]
struct ClientLogFormatter {
    timer: ClientLogTime,
}

impl<S, N> FormatEvent<S, N> for ClientLogFormatter
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> stdfmt::Result {
        self.timer.format_time(&mut writer)?;

        let metadata = event.metadata();
        let thread_name = current_thread_name();
        write!(
            writer,
            " {:<5} [{}] {} - ",
            metadata.level(),
            thread_name,
            metadata.target()
        )?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

#[derive(Default)]
struct ClientLogTime;

impl FormatTime for ClientLogTime {
    fn format_time(&self, writer: &mut Writer<'_>) -> stdfmt::Result {
        let now = now_with_local_offset();
        let formatted = now.format(CLIENT_TIME_FORMAT).map_err(|_| stdfmt::Error)?;
        writer.write_str(&formatted)
    }
}

fn now_with_local_offset() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(current_local_offset())
}

fn current_local_offset() -> UtcOffset {
    *LOCAL_TIME_OFFSET.get_or_init(|| UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC))
}

fn current_thread_name() -> String {
    match thread::current().name() {
        Some(name) if !name.is_empty() => name.to_owned(),
        _ => format!("thread-{:?}", thread::current().id()),
    }
}
