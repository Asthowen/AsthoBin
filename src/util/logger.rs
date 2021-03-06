use env_logger::{fmt::{Color, Style}, Builder, Env};
use std::io::Write;

pub fn init_logger() {
    let env: Env = Env::default()
        .filter_or("MY_LOG_LEVEL", "DEBUG");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style: Style = buf.style();
            if record.level() == log::Level::Error {
                style.set_color(Color::Red).set_bold(true);
            } else if record.level() == log::Level::Warn {
                style.set_color(Color::Yellow).set_bold(true);
            }

            writeln!(
                buf, "[{}] [{}] [{}] {}", style.value(record.level()), style.value(record.target()),
                style.value(chrono::Local::now().format("%d/%m/%Y - %H:%M:%S")),
                style.value(record.args())
            )
        })
        .init();
}