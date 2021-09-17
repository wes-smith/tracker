mod matrix_multiply;

use matrix_multiply::{mm,MMData, init_mat};

pub fn multiply(A: &mut Vec<Vec<usize>>, B: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MMData){
    mm(A,B)
}

pub fn init(size: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>){
    init_mat(size)
} 