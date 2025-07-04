#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

extern crate alloc;

use alloc::string::{String, ToString};

use colorz::Colorize;
use tracing::{Event, Subscriber, field::Field};

struct NoStdSubscriber {
    level: tracing::Level,
}
impl NoStdSubscriber {
    pub fn new(level: tracing::Level) -> Self {
        NoStdSubscriber { level }
    }
}

impl Subscriber for NoStdSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        metadata.level() <= &self.level
    }

    fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut message = String::new();
        let mut visitor = StringVisitor(&mut message);
        event.record(&mut visitor);

        // Output however you need - to RTT, UART, etc.
        let colored = match *event.metadata().level() {
            tracing::Level::ERROR => message.red().to_string(),
            tracing::Level::WARN => message.yellow().to_string(),
            tracing::Level::INFO => message.green().to_string(),
            tracing::Level::DEBUG => message.blue().to_string(),
            tracing::Level::TRACE => message.cyan().to_string(),
        };

        println!(
            "[{}]: file: {}, {}",
            event.metadata().level(),
            event.metadata().file().unwrap_or_default(),
            colored
        );
    }

    fn enter(&self, _span: &tracing::span::Id) {}
    fn exit(&self, _span: &tracing::span::Id) {}
}

struct StringVisitor<'a>(&'a mut String);

impl<'a> tracing::field::Visit for StringVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn core::fmt::Debug) {
        if field.name() == "message" {
            use core::fmt::Write;
            let _ = write!(self.0, "{:?}", value);
        }
    }
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let subscriber = NoStdSubscriber::new(tracing::Level::DEBUG);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    tracing::error!("tracing test, error msg");
    tracing::warn!("tracing test, warn msg");
    tracing::info!("tracing test, info msg");
    tracing::debug!("tracing test, debug msg");
    tracing::trace!("tracing test, trace msg");

    println!("tracing test passed!");
    return 0;
}
