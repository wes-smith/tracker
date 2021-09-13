use nanorand::{Rng, WyRand};

use log::{trace, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use crate::rttrace::Data;
use crate::rttrace::{init,trace};


pub fn matrix_multiply(A: &Vec<Vec<usize>>, B: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let rowA = A.len();
    let colA = A[0].len();
    let rowB = B.len();
    let colB = B[0].len();
    return mat_mult_helper(rowA, colA, A, rowB, colB, B);
}

fn mat_mult_helper(rowA: usize, colA: usize, A: &Vec<Vec<usize>>, rowB: usize, colB: usize, B: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    if rowB != colA {
        println!("Misshaped matrices");
        return A.to_vec();
    }
    let mut C: Vec<Vec<usize>> = vec![vec![0; rowA]; colB];

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    mat_mult_rec(rowA,colA,A,rowB,colB,B,&mut C,&mut i,&mut j,&mut k);
    return C;
}

fn mat_mult_rec(rowA: usize, colA: usize, A: &Vec<Vec<usize>>, rowB: usize, colB: usize, B: &Vec<Vec<usize>>, C: &mut Vec<Vec<usize>>, i: &mut usize, j: &mut usize, k: &mut usize){
    trace!("Read\ti");
    trace!("Read\trowA");
    if *i >= rowA{

        return;
    }
    trace!("Read\tj");
    trace!("Read\tcolB");
    if *j < colB {

        trace!("Read\tk");
        trace!("Read\tcolA");
        if *k < colA {

            trace!("Read\ti");
            trace!("Read\tk");
            trace!("Read\tA[{}][{}]", i, k);
            trace!("Read\tk");
            trace!("Read\tj");
            trace!("Read\tB[{}][{}]", k, j);
            trace!("Read\tC[{}][{}]", i, j);
            trace!("Write\tC[{}][{}]", i, j);
            C[*i][*j] += A[*i][*k] * B[*k][*j];

            trace!("Read\tk");
            trace!("Write\tk");
            *k+=1;

            mat_mult_rec(rowA, colA, A, rowB, colB, B, C, i, j, k);
        }
        trace!("Read\tk");
        *k = 0;
        trace!("Read\tj");
        trace!("Write\th");
        *j += 1;
        mat_mult_rec(rowA, colA, A, rowB, colB, B, C, i, j, k);
    }
    trace!("Read\tj");
    *j = 0;

    trace!("Read\ti");
    trace!("Write\ti");
    *i += 1;

    mat_mult_rec(rowA, colA, A, rowB, colB, B, C, i, j, k);
}

pub fn quick_sort_rt(arr: &mut Vec<i32>) -> Data {
    let mut data = init();


    let low = 0;
    let high = arr.len() as i32;
    quick_sort_helper_rt(arr, low, high - 1, &mut data);
    data
}

fn quick_sort_helper_rt(arr: &mut Vec<i32>, low: i32, high: i32, data: &mut Data) {
    trace("READ\tlow".to_string(), data);
    trace("READ\thigh".to_string(), data);
    if low < high {
        trace("WRITE\tpivot".to_string(), data);
        let pivot = partition_rt(arr, low, high, data);

        trace("READ\tpivot".to_string(), data);
        trace("READ\tarr".to_string(), data);
        trace("READ\tlow".to_string(), data);
        quick_sort_helper_rt(arr, low, pivot - 1, data);

        trace("READ\tpivot".to_string(), data);
        trace("READ\tarr".to_string(), data);
        trace("READ\tlow".to_string(), data);
        quick_sort_helper_rt(arr, pivot + 1, high, data);
    }
}

fn partition_rt(arr: &mut Vec<i32>, low: i32, high: i32, data: &mut Data) -> i32 {
    trace("READ\thigh".to_string(), data);
    trace("WRITE\tpivot".to_string(), data);
    let pivot = high; //rng(low,high);

    trace("READ\tlow".to_string(), data);
    trace("WRITE\tindex".to_string(), data);
    let mut index = low - 1;

    trace("READ\thigh".to_string(), data);
    trace("WRITE\tlast".to_string(), data);
    let mut last = high;

    loop {
        trace("READ\tindex".to_string(), data);
        trace("WRITE\tindex".to_string(), data);
        index += 1;

        trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
        trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
        while arr[index as usize] < arr[pivot as usize] {
            trace("READ\tindex".to_string(), data);
            trace("WRITE\tindex".to_string(), data);
            index += 1;

            trace("READ\tindex".to_string(), data);
            trace("READ\tpivot".to_string(), data);
            trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
            trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
        }

        trace("READ\tlast".to_string(), data);
        trace("WRITE\tlast".to_string(), data);
        last -= 1;

        trace("READ\tlast".to_string(), data);
        trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
        trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
        while last >= 0 && arr[last as usize] > arr[pivot as usize] {
            trace("READ\tindex".to_string(), data);
            trace("WRITE\tindex".to_string(), data);
            last -= 1;

            trace("READ\tlast".to_string(), data);
            trace("READ\tpivot".to_string(), data);
            trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
            trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
        }

        trace("READ\tindex".to_string(), data);
        trace("READ\tlast".to_string(), data);
        if index >= last {
            break;
        } else {
            trace("READ\tindex".to_string(), data);
            trace("WRITE\tlast".to_string(), data);
            trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
            trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
            trace("WRITE\tarr[{".to_string() + &index.to_string() + "}]", data);
            trace("WRITE\tarr[{".to_string() + &last.to_string() + "}]", data);
            arr.swap(index as usize, last as usize);
        }
    }

    trace("READ\tindex".to_string(), data);
    trace("WRITE\tpivot".to_string(), data);
    trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
    trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
    trace("WRITE\tarr[{".to_string() + &index.to_string() + "}]", data);
    trace("WRITE\tarr[{".to_string() + &pivot.to_string() + "}]", data);
    arr.swap(index as usize, pivot as usize);
    index
}




























pub fn quick_sort(arr: &mut Vec<i32>, file_path: &str) {
    let file_path = file_path;

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("trace", Box::new(logfile)))
        .build(Root::builder().appender("trace").build(LevelFilter::Trace))
        .unwrap();

    let _handle = log4rs::init_config(config);

    let low = 0;
    let high = arr.len() as i32;
    quick_sort_helper(arr, low, high - 1);
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: i32, high: i32) {
    trace!("READ\tlow");
    trace!("READ\thigh");
    if low < high {
        trace!("WRITE\tpivot");
        let pivot = partition(arr, low, high);

        trace!("READ\tpivot");
        trace!("READ\tarr");
        trace!("READ\tlow");
        quick_sort_helper(arr, low, pivot - 1);

        trace!("READ\tpivot");
        trace!("READ\tarr");
        trace!("READ\tlow");
        quick_sort_helper(arr, pivot + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    trace!("READ\thigh");
    trace!("WRITE\tpivot");
    let pivot = high; //rng(low,high);

    trace!("READ\tlow");
    trace!("WRITE\tindex");
    let mut index = low - 1;

    trace!("READ\thigh");
    trace!("WRITE\tlast");
    let mut last = high;

    loop {
        trace!("READ\tindex");
        trace!("WRITE\tindex");
        index += 1;

        trace!("READ\tarr[{}]", index);
        trace!("READ\tarr[{}]", pivot);
        while arr[index as usize] < arr[pivot as usize] {
            trace!("READ\tindex");
            trace!("WRITE\tindex");
            index += 1;

            trace!("READ\tindex");
            trace!("READ\tpivot");
            trace!("READ\tarr[{}]", index);
            trace!("READ\tarr[{}]", pivot);
        }

        trace!("READ\tlast");
        trace!("WRITE\tlast");
        last -= 1;

        trace!("READ\tlast");
        trace!("READ\tarr[{}]", last);
        trace!("READ\tarr[{}]", pivot);
        while last >= 0 && arr[last as usize] > arr[pivot as usize] {
            trace!("READ\tindex");
            trace!("WRITE\tindex");
            last -= 1;

            trace!("READ\tlast");
            trace!("READ\tpivot");
            trace!("READ\tarr[{}]", last);
            trace!("READ\tarr[{}]", pivot);
        }

        trace!("READ\tindex");
        trace!("READ\tlast");
        if index >= last {
            break;
        } else {
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

// #[allow(dead_code)]
// fn rng(low: i32, high: i32) -> i32 {
//     let mut rng = WyRand::new();
//     rng.generate_range(low..=high)
// }

pub fn init_arr(size: usize) -> Vec<i32> {
    let mut arr = Vec::new();
    let mut rng = nanorand::tls_rng();

    let low: i32 = -(size as i32) / 2;
    let high: i32 = size as i32 / 2;
    for _i in 0..size {
        arr.push(rng.generate_range(low..=high));
    }
    arr
}

// pub fn shuffle(arr: &mut Vec<i32>) -> Vec<i32> {
//     let mut rng = WyRand::new();
//     rng.shuffle(&mut arr);
//     arr.to_vec()
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn smoke() {
//         assert!(1 == 1);
//     }

//     #[test]
//     fn sort() {
//         let mut arr = vec![8, 7, 6, 5, 3, 2, 1];
//         let mut arr2 = arr.clone();
//         arr2.sort();
//         quick_sort(&mut arr);
//         assert_eq!(arr, arr2);
//     }
// }
