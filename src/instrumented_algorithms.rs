// use nanorand::{Rng, WyRand};

use log::{trace, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
// use crate::rttrace::Data;
// use crate::rttrace::{init,trace};

// pub fn corners_raw(A: &mut Vec<Vec<usize>>) -> (Box<&Vec<Vec<usize>>>, Box<&Vec<Vec<usize>>>, Box<&Vec<Vec<usize>>>, Box<&Vec<Vec<usize>>>){
//     //println!("{:?}\n", A);
//     let len = A.len();
//     let ptr = A.as_mut_ptr();
//     let mid = len/2;
//     let cap = A.capacity();

//     unsafe {
//             let tl_ptr = Box::new(&Vec::from_raw_parts(ptr, mid, cap));
//             let tr_ptr = Box::new(&Vec::from_raw_parts(ptr, mid, cap));
//             let bl_ptr = Box::new(&Vec::from_raw_parts(ptr.offset(mid as isize), mid, cap));
//             let br_ptr = Box::new(&Vec::from_raw_parts(ptr.offset(mid as isize), mid, cap));
//             //return (tl_ptr,tr_ptr,bl_ptr,br_ptr);
//     }
// }
// use std::slice;


// pub fn matrix_multiply_raw(A: &mut Vec<Vec<usize>>, B: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>> {
//     let n = A.len();
//     if n == 1 {
//         return vec![vec![A[0][0] * B[0][0]]]; 
//     }

//     let mut C = vec![vec![0 as usize; n]; n];

//     let len = A.len();
//     let ptr = A.as_mut_ptr();
//     let mid = len/2;
//     let cap = A.capacity();

//     unsafe {
//         let mut top_ptr: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(ptr, mid).to_vec();
//         let mut bottom_ptr: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(ptr.offset(mid as isize), mid).to_vec();
//         let top_ptr = top_ptr.as_mut_ptr();
//         let bottom_ptr = bottom_ptr.as_mut_ptr();
       
//         let tl: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(top_ptr, mid).to_vec();
//         let tr: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(top_ptr.offset(mid as isize), mid).to_vec();
//         //println!("here");
//         let bl: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(bottom_ptr, mid).to_vec();
//         let br: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(bottom_ptr.offset(mid as isize), mid).to_vec();
//         println!("{:?}\n\n{:?}\n{:?}\n{:?}\n{:?}\n", A,tl,tr,bl,br);
//     }

//     // c11 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b21.to_vec())).to_vec();
//     // c12 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b22.to_vec())).to_vec();
//     // c21 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b21.to_vec())).to_vec();
//     // c22 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b22.to_vec())).to_vec();
//     // C = stitch(&c11,&c12,&c21,&c22);
//     C
// }

/*Assuming square matrix & dim is a power of 2  
    https://shivathudi.github.io/jekyll/update/2017/06/15/matr-mult.html
*/
pub fn matrix_multiply(A: &mut Vec<Vec<usize>>, B: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let n = A.len();
    if n == 1 {
        trace!("read\tA[0][0]");
        trace!("read\tB[0][0]");
        println!("read\tA[0][0]");
        println!("read\tB[0][0]");
        return vec![vec![A[0][0] * B[0][0]]]; 
    }

    let mut C = vec![vec![0 as usize; n]; n];

    let (mut a11,mut a12,mut a21,mut a22) = corners(&A); //deal with temp memory
    let (mut b11,mut b12,mut b21,mut b22) = corners(&B);
    let (mut c11,mut c12,mut c21,mut c22) = corners(&C);

    c11 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b21.to_vec())).to_vec();
    c12 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b22.to_vec())).to_vec();
    c21 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b21.to_vec())).to_vec();
    c22 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b22.to_vec())).to_vec();
    C = stitch(&c11,&c12,&c21,&c22);
    C
}

fn matrix_add(A: &Vec<Vec<usize>>, B: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let n = A.len();
    let mut C = vec![vec![0 as usize; n]; n];

    for (i,row) in C.iter_mut().enumerate(){
        for (j,element) in row.iter_mut().enumerate(){
            *element = A[i][j] + B[i][j];
        }
    }
    C
}

pub fn stitch(tl: &Vec<Vec<usize>>, tr: &Vec<Vec<usize>>, bl: &Vec<Vec<usize>>, br: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = tl.len();
    let mut C = Vec::new();

    for i in 0..(2*n) {
        let mut row = Vec::new();
        for j in 0..(2*n) {
            if i <= n/2 && j <= n/2{ //tl
                row.push(tl[i/2][j/2]);
            }
            else if i <= n/2 && j >= n/2{ //tr
                row.push(tr[i/2][j/2]);
            }
            else if i >= n/2 && j <= n/2{ //bl
                row.push(bl[i/2][j/2]);
            }
            else if i >= n/2 && j >= n/2{ //br
                row.push(br[i/2][j/2]);
            }
            else{
                break; //unreachable
            }
        }
        C.push(row);
    }
    C
}

pub fn corners(A: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>){
    //println!("{:?}\n", A);
    let n = A.len();

    let mut tl: Vec<Vec<usize>> = Vec::new();
    let mut tr: Vec<Vec<usize>> = Vec::new();
    let mut bl: Vec<Vec<usize>> = Vec::new();
    let mut br: Vec<Vec<usize>> = Vec::new();

    if n == 1 {
        tl.push(vec![A[0][0]]);
        return (tl,tr,bl,br);
    }

    for i in (0..n){
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for j in (0..n){
            if i < n/2 && j < n/2{ //tl
                left.push(A[i][j]);
            }
            else if i < n/2 && j > n/2{ //tr
                right.push(A[i][j]);
            }
            else if i > n/2-1 && j < n/2{ //bl
                left.push(A[i][j]);
            }
            else{ //br
                right.push(A[i][j]);
            }
        }
        if i < n/2 {
            tl.push(left);
            tr.push(right);
        }
        else{
            bl.push(left);
            br.push(right);
        }
    }

    return (tl,tr,bl,br);
}

// pub fn quick_sort_rt(arr: &mut Vec<i32>) -> Data {
//     let mut data = init();


//     let low = 0;
//     let high = arr.len() as i32;
//     quick_sort_helper_rt(arr, low, high - 1, &mut data);
//     data
// }

// fn quick_sort_helper_rt(arr: &mut Vec<i32>, low: i32, high: i32, data: &mut Data) {
//     trace("READ\tlow".to_string(), data);
//     trace("READ\thigh".to_string(), data);
//     if low < high {
//         trace("WRITE\tpivot".to_string(), data);
//         let pivot = partition_rt(arr, low, high, data);

//         trace("READ\tpivot".to_string(), data);
//         trace("READ\tarr".to_string(), data);
//         trace("READ\tlow".to_string(), data);
//         quick_sort_helper_rt(arr, low, pivot - 1, data);

//         trace("READ\tpivot".to_string(), data);
//         trace("READ\tarr".to_string(), data);
//         trace("READ\tlow".to_string(), data);
//         quick_sort_helper_rt(arr, pivot + 1, high, data);
//     }
// }

// fn partition_rt(arr: &mut Vec<i32>, low: i32, high: i32, data: &mut Data) -> i32 {
//     trace("READ\thigh".to_string(), data);
//     trace("WRITE\tpivot".to_string(), data);
//     let pivot = high; //rng(low,high);

//     trace("READ\tlow".to_string(), data);
//     trace("WRITE\tindex".to_string(), data);
//     let mut index = low - 1;

//     trace("READ\thigh".to_string(), data);
//     trace("WRITE\tlast".to_string(), data);
//     let mut last = high;

//     loop {
//         trace("READ\tindex".to_string(), data);
//         trace("WRITE\tindex".to_string(), data);
//         index += 1;

//         trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
//         trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//         while arr[index as usize] < arr[pivot as usize] {
//             trace("READ\tindex".to_string(), data);
//             trace("WRITE\tindex".to_string(), data);
//             index += 1;

//             trace("READ\tindex".to_string(), data);
//             trace("READ\tpivot".to_string(), data);
//             trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
//             trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//         }

//         trace("READ\tlast".to_string(), data);
//         trace("WRITE\tlast".to_string(), data);
//         last -= 1;

//         trace("READ\tlast".to_string(), data);
//         trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
//         trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//         while last >= 0 && arr[last as usize] > arr[pivot as usize] {
//             trace("READ\tindex".to_string(), data);
//             trace("WRITE\tindex".to_string(), data);
//             last -= 1;

//             trace("READ\tlast".to_string(), data);
//             trace("READ\tpivot".to_string(), data);
//             trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
//             trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//         }

//         trace("READ\tindex".to_string(), data);
//         trace("READ\tlast".to_string(), data);
//         if index >= last {
//             break;
//         } else {
//             trace("READ\tindex".to_string(), data);
//             trace("WRITE\tlast".to_string(), data);
//             trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
//             trace("READ\tarr[{".to_string() + &last.to_string() + "}]", data);
//             trace("WRITE\tarr[{".to_string() + &index.to_string() + "}]", data);
//             trace("WRITE\tarr[{".to_string() + &last.to_string() + "}]", data);
//             arr.swap(index as usize, last as usize);
//         }
//     }

//     trace("READ\tindex".to_string(), data);
//     trace("WRITE\tpivot".to_string(), data);
//     trace("READ\tarr[{".to_string() + &index.to_string() + "}]", data);
//     trace("READ\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//     trace("WRITE\tarr[{".to_string() + &index.to_string() + "}]", data);
//     trace("WRITE\tarr[{".to_string() + &pivot.to_string() + "}]", data);
//     arr.swap(index as usize, pivot as usize);
//     index
// }


// pub fn quick_sort(arr: &mut Vec<i32>, file_path: &str) {
//     let file_path = file_path;

//     let logfile = FileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new("{m}\n")))
//         .build(file_path)
//         .unwrap();

//     let config = Config::builder()
//         .appender(Appender::builder().build("trace", Box::new(logfile)))
//         .build(Root::builder().appender("trace").build(LevelFilter::Trace))
//         .unwrap();

//     let _handle = log4rs::init_config(config);

//     let low = 0;
//     let high = arr.len() as i32;
//     quick_sort_helper(arr, low, high - 1);
// }

// fn quick_sort_helper(arr: &mut Vec<i32>, low: i32, high: i32) {
//     trace!("READ\tlow");
//     trace!("READ\thigh");
//     if low < high {
//         trace!("WRITE\tpivot");
//         let pivot = partition(arr, low, high);

//         trace!("READ\tpivot");
//         trace!("READ\tarr");
//         trace!("READ\tlow");
//         quick_sort_helper(arr, low, pivot - 1);

//         trace!("READ\tpivot");
//         trace!("READ\tarr");
//         trace!("READ\tlow");
//         quick_sort_helper(arr, pivot + 1, high);
//     }
// }

// fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
//     trace!("READ\thigh");
//     trace!("WRITE\tpivot");
//     let pivot = high; //rng(low,high);

//     trace!("READ\tlow");
//     trace!("WRITE\tindex");
//     let mut index = low - 1;

//     trace!("READ\thigh");
//     trace!("WRITE\tlast");
//     let mut last = high;

//     loop {
//         trace!("READ\tindex");
//         trace!("WRITE\tindex");
//         index += 1;

//         trace!("READ\tarr[{}]", index);
//         trace!("READ\tarr[{}]", pivot);
//         while arr[index as usize] < arr[pivot as usize] {
//             trace!("READ\tindex");
//             trace!("WRITE\tindex");
//             index += 1;

//             trace!("READ\tindex");
//             trace!("READ\tpivot");
//             trace!("READ\tarr[{}]", index);
//             trace!("READ\tarr[{}]", pivot);
//         }

//         trace!("READ\tlast");
//         trace!("WRITE\tlast");
//         last -= 1;

//         trace!("READ\tlast");
//         trace!("READ\tarr[{}]", last);
//         trace!("READ\tarr[{}]", pivot);
//         while last >= 0 && arr[last as usize] > arr[pivot as usize] {
//             trace!("READ\tindex");
//             trace!("WRITE\tindex");
//             last -= 1;

//             trace!("READ\tlast");
//             trace!("READ\tpivot");
//             trace!("READ\tarr[{}]", last);
//             trace!("READ\tarr[{}]", pivot);
//         }

//         trace!("READ\tindex");
//         trace!("READ\tlast");
//         if index >= last {
//             break;
//         } else {
//             trace!("READ\tindex");
//             trace!("WRITE\tlast");
//             trace!("READ\tarr[{}]", index);
//             trace!("READ\tarr[{}]", last);
//             trace!("WRITE\tarr[{}]", index);
//             trace!("WRITE\tarr[{}]", last);
//             arr.swap(index as usize, last as usize);
//         }
//     }

//     trace!("READ\tindex");
//     trace!("WRITE\tpivot");
//     trace!("READ\tarr[{}]", index);
//     trace!("READ\tarr[{}]", pivot);
//     trace!("WRITE\tarr[{}]", index);
//     trace!("WRITE\tarr[{}]", pivot);
//     arr.swap(index as usize, pivot as usize);
//     index
// }

// #[allow(dead_code)]
// fn rng(low: i32, high: i32) -> i32 {
//     let mut rng = WyRand::new();
//     rng.generate_range(low..=high)
// }

// pub fn init_arr(size: usize) -> Vec<i32> {
//     let mut arr = Vec::new();
//     let mut rng = nanorand::tls_rng();

//     let low: i32 = -(size as i32) / 2;
//     let high: i32 = size as i32 / 2;
//     for _i in 0..size {
//         arr.push(rng.generate_range(low..=high));
//     }
//     arr
// }

// pub fn shuffle(arr: &mut Vec<i32>) -> Vec<i32> {
//     let mut rng = WyRand::new();
//     rng.shuffle(&mut arr);
//     arr.to_vec()
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn stitch_corners() {
        let mut A = Vec::new();
        A.push(vec![1,2]);
        A.push(vec![3,4]);

        let (tl,tr,bl,br) = corners(&A);
        let B = stitch(&tl,&tr,&bl,&br);

        assert_eq!(A, B);
    }
}
