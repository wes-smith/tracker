use log::{trace, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
mod lib;

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
    // log4rs::init_config(config).unwrap();

    // let arr = vec![0,1,2,3,4,5,6,7,8,9];
    // let mut _sum = 0;
    // for i in 0..10 {
    //     _sum += arr[i];
    //     trace!("sum: {:?} ", _sum);
    // }
    let arr = vec!["a","b","c","a","b","c"];
    let res = lib::reuse_interval(arr);
    println!("{:?}", res);

    Ok(())
}
