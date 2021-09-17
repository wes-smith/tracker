use crate::rttrace::Data;
use crate::rttrace::{init,trace};


pub fn mm(A: &mut Vec<Vec<usize>>, B: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Data){
    let mut data = init();
    (matrix_multiply(A, B, &mut data), data)
}

/*Assuming square matrix & dim is a power of 2  
    https://shivathudi.github.io/jekyll/update/2017/06/15/matr-mult.html
*/
pub fn matrix_multiply(A: &mut Vec<Vec<usize>>, B: &mut Vec<Vec<usize>>, data: &mut Data) -> Vec<Vec<usize>>{
    let n = A.len();
    if n == 1 {
        let ptr_a = &A[0][0];
        let ptr_b = &B[0][0];
        trace(ptr_a.to_string(),data);
        trace(ptr_b.to_string(),data);
        return vec![vec![A[0][0] * B[0][0]]]; 
    }

    let mut C = vec![vec![0 as usize; n]; n];

    let (mut a11,mut a12,mut a21,mut a22) = corners(&A); //deal with temp memory
    let (mut b11,mut b12,mut b21,mut b22) = corners(&B);
    let (mut c11,mut c12,mut c21,mut c22) = corners(&C);

    c11 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b11.to_vec(), data), &mut matrix_multiply(&mut a12.to_vec(), &mut b21.to_vec(), data), data).to_vec();
    c12 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b12.to_vec(), data), &mut matrix_multiply(&mut a12.to_vec(), &mut b22.to_vec(), data), data).to_vec();
    c21 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b11.to_vec(), data), &mut matrix_multiply(&mut a22.to_vec(), &mut b21.to_vec(), data), data).to_vec();
    c22 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b12.to_vec(), data), &mut matrix_multiply(&mut a22.to_vec(), &mut b22.to_vec(), data), data).to_vec();
    C = stitch(&c11,&c12,&c21,&c22);
    C
}

fn matrix_add(A: &Vec<Vec<usize>>, B: &Vec<Vec<usize>>, data: &mut Data) -> Vec<Vec<usize>>{
    let n = A.len();
    let mut C = vec![vec![0 as usize; n]; n];

    for (i,row) in C.iter_mut().enumerate(){
        for (j,element) in row.iter_mut().enumerate(){
            let ptr_a = &A[0][0];
            let ptr_b = &B[0][0];
            trace(ptr_a.to_string(),data);
            trace(ptr_b.to_string(),data);
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

       
//         let tl = &mut slice::from_raw_parts_mut(top_ptr, mid);
//         println!("tl: {:?}", tl);
//         // let tr: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(top_ptr.offset(mid as isize), mid).to_vec(); //seg faults due to offset
//         // let bl: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(bottom_ptr, mid).to_vec();
//         // let br: &mut Vec<Vec<usize>> = &mut slice::from_raw_parts_mut(bottom_ptr.offset(mid as isize), mid).to_vec();
//         // println!("{:?}\n\n{:?}\n{:?}\n{:?}\n{:?}\n", A,tl,tr,bl,br);
//     }

    // c11 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b21.to_vec())).to_vec();
    // c12 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a12.to_vec(), &mut b22.to_vec())).to_vec();
    // c21 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b11.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b21.to_vec())).to_vec();
    // c22 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b12.to_vec()), &mut matrix_multiply(&mut a22.to_vec(), &mut b22.to_vec())).to_vec();
    // C = stitch(&c11,&c12,&c21,&c22);
//     C
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