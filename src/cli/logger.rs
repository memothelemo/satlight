use ansi_term::{Color, Style};

pub struct CLILogger;

impl log::Log for CLILogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            macro_rules! precfg {
                ($color:expr, $text:expr) => {
                    Style::new().bold().fg($color).paint($text)
                };
            }
            println!(
                "[{}]: {}",
                match record.level() {
                    log::Level::Error => precfg!(Color::Red, "ERR"),
                    log::Level::Warn => precfg!(Color::Yellow, "WRN"),
                    log::Level::Info => precfg!(Color::White, "INF"),
                    log::Level::Debug => precfg!(Color::Blue, "DBG"),
                    log::Level::Trace => precfg!(Color::Cyan, "TRC"),
                },
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
