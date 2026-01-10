use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

const FILE_APPENDER_NAME: &str = "file";
const CONSOLE_APPENDER_NAME: &str = "console";

const LOG_FILE_PATH: &str = "pw.log";

pub fn get_logging_config(logging_level: &str, log_target: &str) -> Config {
    let level = get_logging_level_from_string(logging_level);

    match log_target {
        "file" => Config::builder()
            .appender(get_rolling_appender(level))
            .logger(get_default_logger(level))
            .build(Root::builder().appender(FILE_APPENDER_NAME).build(level))
            .unwrap_or_else(|_| panic!("unable to create log file '{}'", LOG_FILE_PATH)),
        _ => Config::builder()
            .appender(get_console_appender(level))
            .logger(get_default_logger(level))
            .build(Root::builder().appender(CONSOLE_APPENDER_NAME).build(level))
            .expect("unable to create console logging configuration"),
    }
}

fn get_logging_level_from_string(level: &str) -> LevelFilter {
    match level {
        "debug" => LevelFilter::Debug,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "trace" => LevelFilter::Trace,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    }
}

fn get_rolling_appender(level: LevelFilter) -> Appender {
    let log_file_format = format!("{}.{{}}", LOG_FILE_PATH);

    let fixed_window_roller = FixedWindowRoller::builder()
        .build(&log_file_format, 5)
        .expect("couldn't build fixed window roller");

    let size_trigger = SizeTrigger::new(100_000_000);
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
    let rolling_appender = RollingFileAppender::builder()
        .encoder(get_encoder())
        .build(LOG_FILE_PATH, Box::new(policy))
        .expect("couldn't build rolling appender");

    Appender::builder()
        .filter(Box::new(ThresholdFilter::new(level)))
        .build(FILE_APPENDER_NAME, Box::new(rolling_appender))
}

fn get_encoder() -> Box<PatternEncoder> {
    Box::new(PatternEncoder::new(
        "{d(%Y-%m-%d %H:%M:%S)} - {l} - [{M}] - {m}{n}",
    ))
}

fn get_console_appender(level: LevelFilter) -> Appender {
    let console_appender = ConsoleAppender::builder().encoder(get_encoder()).build();

    Appender::builder()
        .filter(Box::new(ThresholdFilter::new(level)))
        .build(CONSOLE_APPENDER_NAME, Box::new(console_appender))
}

fn get_default_logger(level: LevelFilter) -> Logger {
    Logger::builder().build("default", level)
}
