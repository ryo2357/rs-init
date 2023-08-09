use chrono::{DateTime, Local};
use log::LevelFilter;
use log4rs::{
    append::console::{ConsoleAppender, Target},
    filter::threshold::ThresholdFilter,
};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

pub fn setup_logger() {
    let now: DateTime<Local> = Local::now();
    // CHECK:ログの名前に実行ファイル名を入れたかったが取得方法が思いつかないので保留
    // examplesの実行ログについて分別を行いたい
    let log_file_path = "log/".to_string() + &now.format("%Y-%m-%d_%H%M%S").to_string() + ".log";
    make_logger(&log_file_path);
}

#[cfg(debug_assertions)]
fn make_logger(log_file_path: &str) {
    // ファイル・コマンドライン
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{l} - {f},{L} - {m}{n}")))
        .build();

    // ファイル出力の設定
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f %Z)},{h({l})},{f},{L},{m}{n}",
        )))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("logfile", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[cfg(not(debug_assertions))]
fn make_logger(log_file_path: &str) {
    // ファイル・コマンドライン
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{l} - {f},{L} - {m}{n}")))
        .build();

    // ファイル出力の設定
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f %Z)},{h({l})},{f},{L},{m}{n}",
        )))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("logfile", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Warn)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
