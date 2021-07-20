use log::{trace, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

fn main() -> Result<(), SetLoggerError> {
    let file_path = "foo.log";

    let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("trace", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("trace")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    for i in 0..10 {
        trace!("{}", i.to_string());
    }

    Ok(())
}
