mod matrix_multiply;

use matrix_multiply::{mm,MMData, init_mat};

pub fn multiply(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MMData){
    mm(a,b)
}

pub fn init(size: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>){
    init_mat(size)
} 