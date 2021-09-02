use nanorand::{Rng, WyRand};

use log::{trace, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};


pub fn quick_sort(arr: &mut Vec<i32>){
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

    let _handle = log4rs::init_config(config);

    // for i in 0..10 {
    //     trace!("{}", i.to_string());
    // }



    let low = 0;
    let high = arr.len() as i32;
    trace!("READ\tarr");
    quick_sort_helper(arr, low, high-1);
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: i32, high: i32){
    if low < high{
        trace!("READ\tlow");
        trace!("READ\thigh");
        let pivot = partition(arr,low,high);
        trace!("WRITE\tpivot");
        quick_sort_helper(arr, low, pivot-1);
        quick_sort_helper(arr, pivot+1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32{
    let pivot = high;//rng(low,high);
    trace!("READ\thigh");
    let mut index = low - 1;
    trace!("READ\tlow");
    let mut last = high;
    trace!("READ\thigh");

    loop {
        index+=1;
        trace!("READ\tindex");
        trace!("WRITE\tindex");
        while arr[index as usize] < arr[pivot as usize] {
            trace!("READ\tarr[{}]", index);
            trace!("READ\tarr[{}]", pivot);
            index+=1;
            trace!("READ\tindex");
            trace!("WRITE\tindex");
        }
        last -= 1;
        trace!("READ\tlast");
        trace!("WRITE\tlast");
        while last >= 0 && arr[last as usize] > arr[pivot as usize]{
            trace!("READ\tlast");
            trace!("READ\tarr[{}]", last);
            trace!("READ\tarr[{}]", pivot);
            last -= 1;
            trace!("READ\tindex");
            trace!("WRITE\tindex");
        }
        if index >= last{
            trace!("READ\tindex");
            trace!("READ\tlast");
            break;
        }
        else{
            arr.swap(index as usize, last as usize);
            trace!("WRITE\tarr[{}]", index);
            trace!("WRITE\tarr[{}]", last);
        }
    }
    arr.swap(index as usize, pivot as usize);
    trace!("WRITE\tarr[{}]", index);
    trace!("WRITE\tarr[{}]", pivot);
    index
}

fn rng(low: i32, high: i32) -> i32{
    let mut rng = WyRand::new();
    rng.generate_range(low..=high)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn sort() {
        let mut arr = vec![8,7,6,5,3,2,1];
        let mut arr2 = arr.clone();
        arr2.sort();
        quick_sort(&mut arr);
        assert_eq!(arr,arr2);
    }
}