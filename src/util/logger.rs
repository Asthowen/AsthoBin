use env_logger::{Builder, Env};
use std::io::Write;

pub fn init_logger() {
    let env: Env = Env::default().filter_or("MY_LOG_LEVEL", "DEBUG,actix_server::builder=off,actix_server::server=off,actix_server::worker=off,actix_server::accept=off,h2=off");

    Builder::from_env(env)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "{level_style}[{}] [{}] [{}] {}{level_style:#}",
                record.level(),
                record.target(),
                chrono::Local::now().format("%d/%m/%Y - %H:%M:%S"),
                record.args()
            )
        })
        .init();
}
