use crate::rttrace::{MMData};
use crate::rttrace::{init,trace};

// #[derive(Debug)]
// pub struct MMData{
//     pub a_b: Data,
//     pub c: Data,
//     pub temp: Data,
// }


pub fn mm(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>) -> (Vec<Vec<usize>>, MMData){
    let a_b = init();
    let c = init();
    let temp = init();
    let mut mmdata = MMData {
        a_b,
        c,
        temp
    };


    (matrix_multiply(a, b, &mut mmdata), mmdata)
}

/*Assuming square matrix & dim is a power of 2  
    https://shivathudi.github.io/jekyll/update/2017/06/15/matr-mult.html
*/
pub fn matrix_multiply(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>, mmdata: &mut MMData) -> Vec<Vec<usize>>{
    trace("A.len".to_string(), &mut mmdata.a_b);
    let n = a.len();

    if n == 1 {
        trace(a[0][0].to_string(), &mut mmdata.a_b);
        trace(b[0][0].to_string(), &mut mmdata.a_b);
        return vec![vec![a[0][0] * b[0][0]]]; 
    }

    //let mut c = vec![vec![0 as usize; n]; n];

    let (a11, a12, a21, a22) = corners(&a, "A", mmdata); //deal with temp memory
    let (b11, b12, b21, b22) = corners(&b, "B", mmdata);
    let (c11, c12, c21, c22);// = corners(&C);

    c11 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b11.to_vec(), mmdata), &mut matrix_multiply(&mut a12.to_vec(), &mut b21.to_vec(), mmdata), mmdata).to_vec();
    c12 = matrix_add(&matrix_multiply(&mut a11.to_vec(), &mut b12.to_vec(), mmdata), &mut matrix_multiply(&mut a12.to_vec(), &mut b22.to_vec(), mmdata), mmdata).to_vec();
    c21 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b11.to_vec(), mmdata), &mut matrix_multiply(&mut a22.to_vec(), &mut b21.to_vec(), mmdata), mmdata).to_vec();
    c22 = matrix_add(&matrix_multiply(&mut a21.to_vec(), &mut b12.to_vec(), mmdata), &mut matrix_multiply(&mut a22.to_vec(), &mut b22.to_vec(), mmdata), mmdata).to_vec();
    let c = stitch(&c11,&c12,&c21,&c22, mmdata);
    c
}

fn matrix_add(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, mmdata: &mut MMData) -> Vec<Vec<usize>>{
    let n = a.len();
    let mut c = vec![vec![0 as usize; n]; n];

    for (i,row) in c.iter_mut().enumerate(){
        for (j,element) in row.iter_mut().enumerate(){
            trace(a[i][j].to_string(), &mut mmdata.a_b);
            trace(b[i][j].to_string(), &mut mmdata.a_b);

            trace("C[".to_string() + &i.to_string() + &"][".to_string() + &j.to_string() + &"]".to_string(), &mut mmdata.c);
            *element = a[i][j] + b[i][j];
        }
    }
    c
}

pub fn stitch(tl: &Vec<Vec<usize>>, tr: &Vec<Vec<usize>>, bl: &Vec<Vec<usize>>, br: &Vec<Vec<usize>>, mmdata: &mut MMData) -> Vec<Vec<usize>> {
    let n = tl.len();
    let mut c = Vec::new();

    for i in 0..(2*n) {
        let mut row = Vec::new();
        for j in 0..(2*n) {
            //println!("{:?}","C[".to_string() + &i.to_string() + &"][".to_string() + &j.to_string() + &"]".to_string());
            trace("C[".to_string() + &i.to_string() + &"][".to_string() + &j.to_string() + &"]".to_string(), &mut mmdata.c);
            trace("temp[".to_string() + &i.to_string() + &"][".to_string() + &j.to_string() + &"]".to_string(), &mut mmdata.temp);
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
        c.push(row);
    }
    c
}

pub fn corners(a: &Vec<Vec<usize>>, id: &str, mmdata: &mut MMData) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>){
    trace(id.to_string() + &".len".to_string(), &mut mmdata.a_b);
    let n = a.len();

    let mut tl: Vec<Vec<usize>> = Vec::new();
    let mut tr: Vec<Vec<usize>> = Vec::new();
    let mut bl: Vec<Vec<usize>> = Vec::new();
    let mut br: Vec<Vec<usize>> = Vec::new();

    if n == 1 {
        trace(id.to_string() + &"[0][0]", &mut mmdata.a_b);
        tl.push(vec![a[0][0]]);
        return (tl,tr,bl,br);
    }

    for i in 0..n{
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for j in 0..n{
            trace(id.to_string() + &"[".to_string() + &i.to_string() + &"][".to_string() + &j.to_string() + &"]".to_string(), &mut mmdata.a_b);
            if i < n/2 && j < n/2{ //tl
                left.push(a[i][j]);
            }
            else if i < n/2 && j > n/2{ //tr
                right.push(a[i][j]);
            }
            else if i > n/2-1 && j < n/2{ //bl
                left.push(a[i][j]);
            }
            else{ //br
                right.push(a[i][j]);
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

pub fn init_mat(size: usize) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 1..size+1{
        let mut a_row = Vec::new();
        let mut b_row = Vec::new();
        for j in 0..size{
            a_row.push(size*i+j);
            b_row.push(size*i+j+size*size);
        }
        a.push(a_row);
        b.push(b_row);
    }
    (a,b)
}

pub fn init_mat_i32(size: i32) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 1..size+1{
        let mut a_row = Vec::new();
        let mut b_row = Vec::new();
        for j in 0..size{
            a_row.push(size*i+j);
            b_row.push(size*i+j+size*size);
        }
        a.push(a_row);
        b.push(b_row);
    }
    (a,b)
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

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn smoke() {
//         assert!(1 == 1);
//     }

//     #[test]
//     fn stitch_corners() {
//         let mut A = Vec::new();
//         A.push(vec![1,2]);
//         A.push(vec![3,4]);

//         let (tl,tr,bl,br) = corners(&A);
//         let B = stitch(&tl,&tr,&bl,&br, _);

//         assert_eq!(A, B);
//     }
// }