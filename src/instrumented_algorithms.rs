mod matrix_multiply;
mod quick_sort;
mod strassen;

use matrix_multiply::{mm, init_mat,init_mat_i32};
use quick_sort::{quick_sort_rt,init_arr};
use strassen::{strassen_mm};
use crate::rttrace::{Data,MMData};

pub fn strassen(a:&mut Vec<Vec<i32>>, b: &mut Vec<Vec<i32>>) -> (Vec<Vec<i32>>, MMData){
    strassen_mm(a,b)
}

pub fn multiply(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MMData){
    mm(a,b)
}

pub fn init(size: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>){
    init_mat(size)
} 

pub fn init_i32(size: i32) -> (Vec<Vec<i32>>, Vec<Vec<i32>>){
    init_mat_i32(size)
} 

pub fn qs(arr: &mut Vec<i32>) -> Data{
    quick_sort_rt(arr)
}

pub fn init_ar(size: usize) -> Vec<i32>{
    init_arr(size)
}  