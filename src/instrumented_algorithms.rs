mod matrix_multiply;
mod quick_sort;

use matrix_multiply::{mm,MMData, init_mat};
use quick_sort::{quick_sort_rt,init_arr};
use crate::rttrace::{Data};

pub fn multiply(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MMData){
    mm(a,b)
}

pub fn init(size: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>){
    init_mat(size)
} 

pub fn qs(arr: &mut Vec<i32>) -> Data{
    quick_sort_rt(arr)
}

pub fn init_ar(size: usize) -> Vec<i32>{
    init_arr(size)
}  