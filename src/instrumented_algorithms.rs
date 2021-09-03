use nanorand::{Rng, WyRand};

use log::{trace, LevelFilter};
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



    let low = 0;
    let high = arr.len() as i32;
    quick_sort_helper(arr, low, high-1);
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: i32, high: i32){
    trace!("READ\tlow");
    trace!("READ\thigh");
    if low < high{

        trace!("WRITE\tpivot");
        let pivot = partition(arr,low,high);
        
        trace!("READ\tpivot");
        trace!("READ\tarr");
        trace!("READ\tlow");
        quick_sort_helper(arr, low, pivot-1);

        trace!("READ\tpivot");
        trace!("READ\tarr");
        trace!("READ\tlow");
        quick_sort_helper(arr, pivot+1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32{

    trace!("READ\thigh");
    trace!("WRITE\tpivot");
    let pivot = high;//rng(low,high);

    trace!("READ\tlow");
    trace!("WRITE\tindex");
    let mut index = low - 1;

    trace!("READ\thigh");
    trace!("WRITE\tlast");
    let mut last = high;

    loop {

        trace!("READ\tindex");
        trace!("WRITE\tindex");
        index+=1;
        
        trace!("READ\tarr[{}]", index);
        trace!("READ\tarr[{}]", pivot);
        while arr[index as usize] < arr[pivot as usize] {
            
            trace!("READ\tindex");
            trace!("WRITE\tindex");
            index+=1;

            trace!("READ\tarr[{}]", index);
            trace!("READ\tarr[{}]", pivot);
        }

        trace!("READ\tlast");
        trace!("WRITE\tlast");
        last -= 1;
        
        trace!("READ\tlast");
        trace!("READ\tarr[{}]", last);
        trace!("READ\tarr[{}]", pivot);
        while last >= 0 && arr[last as usize] > arr[pivot as usize]{
            
            trace!("READ\tindex");
            trace!("WRITE\tindex");
            last -= 1;
            
            trace!("READ\tlast");
            trace!("READ\tarr[{}]", last);
            trace!("READ\tarr[{}]", pivot);
        }

        trace!("READ\tindex");
        trace!("READ\tlast");
        if index >= last{
            break;
        }

        else{
            trace!("READ\tindex");
            trace!("WRITE\tlast");
            trace!("READ\tarr[{}]", index);
            trace!("READ\tarr[{}]", last);
            trace!("WRITE\tarr[{}]", index);
            trace!("WRITE\tarr[{}]", last);
            arr.swap(index as usize, last as usize);
        }
    }

    trace!("READ\tindex");
    trace!("WRITE\tpivot");
    trace!("READ\tarr[{}]", index);
    trace!("READ\tarr[{}]", pivot);
    trace!("WRITE\tarr[{}]", index);
    trace!("WRITE\tarr[{}]", pivot);
    arr.swap(index as usize, pivot as usize);
    index
}

#[allow(dead_code)]
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