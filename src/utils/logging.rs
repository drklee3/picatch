use chrono;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
use std::sync::mpsc::channel;
use std::thread;

use crate::{error::Result, model::config::AppConfig};

pub fn setup_logger(config: &AppConfig) -> Result<()> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        // Does this even help I don't know
        while let Ok(msg) = rx.recv() {
            handle_log_message(msg);
        }
    });

    let colors = ColoredLevelConfig::new()
        .info(Color::BrightGreen)
        .debug(Color::BrightCyan)
        .trace(Color::BrightMagenta);

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
        .level(config.log)
        // senders are async and won't block the main thread
        .chain(tx)
        .apply()?;

    Ok(())
}

fn handle_log_message(msg: String) {
    print!("{}", msg);
}
