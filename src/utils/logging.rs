use crate::error::Result;


use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::env;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

pub fn setup_logger() -> Result<()> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            handle_log_message(msg);
        }
    });

    let colors = ColoredLevelConfig::new()
        .info(Color::BrightGreen)
        .debug(Color::BrightCyan)
        .trace(Color::BrightMagenta);

    let log_level = LevelFilter::from_str(&env::var("PICATCH_LOG").unwrap_or("INFO".to_string()))
        .unwrap_or(LevelFilter::Info);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y%m%d | %H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log_level)
        // senders are async and won't block the main thread
        .chain(tx)
        .apply()?;

    info!("Log filtering: {}", log_level);

    Ok(())
}

fn handle_log_message(msg: String) {
    print!("{}", msg);
}
