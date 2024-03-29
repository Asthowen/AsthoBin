use env_logger::{
    fmt::{Color, Style},
    Builder, Env,
};
use std::io::Write;

pub fn init_logger() {
    let env: Env = Env::default().filter_or("MY_LOG_LEVEL", "DEBUG,actix_server::builder=off,actix_server::server=off,actix_server::worker=off,actix_server::accept=off,h2=off");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style: Style = buf.style();
            if record.level() == log::Level::Error {
                style.set_color(Color::Red).set_bold(true);
            } else if record.level() == log::Level::Warn {
                style.set_color(Color::Yellow).set_bold(true);
            }

            writeln!(
                buf,
                "{}",
                style.value(format!(
                    "[{}] [{}] [{}] {}",
                    record.level(),
                    record.target(),
                    chrono::Local::now().format("%d/%m/%Y - %H:%M:%S"),
                    record.args()
                )),
            )
        })
        .init();
}
