use crate::error::Result;
use chrono;
use fern;
use fern::colors::{Color, ColoredLevelConfig};
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
        .level(log::LevelFilter::Debug)
        // senders are async and won't block the main thread
        .chain(tx)
        .apply()?;

    Ok(())
}

fn handle_log_message(msg: String) {
    print!("{}", msg);
}
