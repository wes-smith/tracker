use log::{trace, LevelFilter, SetLoggerError};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
mod lib;

fn main() -> Result<(), SetLoggerError> {
    let file_path = "foo.log";

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("trace", Box::new(logfile)))
        .build(Root::builder().appender("trace").build(LevelFilter::Trace))
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    let arr = vec![0,1,2,3,4,5,6,7,8,9];
    let mut _sum = 0;
    for item in arr.iter() {
        _sum += item;
        trace!("sum: {:?} ", _sum);
    }


    let arr2 = vec!["a", "b", "c", "a", "b", "c"];
    let (res, _map) = lib::reuse_interval(arr2);
    println!("{:?}", res);

    Ok(())
}
