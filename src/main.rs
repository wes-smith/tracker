mod instrumented_algorithms;
// mod lib;
// mod parse;
// mod rttrace;

// use instrumented_algorithms::init_arr;
use instrumented_algorithms::{matrix_multiply};

// use crate::rttrace::Data;
// use crate::rttrace::{init,trace};

// use std::env;
// use std::fs::File;
// use std::io::prelude::*;

// use nanorand::{Rng, WyRand};

fn corners(A: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>){
    let n = A.len();
    let mut tl: Vec<Vec<usize>> = Vec::new();
    let mut tr: Vec<Vec<usize>> = Vec::new();
    let mut bl: Vec<Vec<usize>> = Vec::new();
    let mut br: Vec<Vec<usize>> = Vec::new();

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

fn main() -> std::io::Result<()> {
    let mut A = Vec::new();
    A.push(vec![1,2]);
    A.push(vec![3,4]);
    let mut B = Vec::new();
    B.push(vec![5,6]);
    B.push(vec![7,8]);
    // let mut A = Vec::new();
    // A.push(vec![1,2,3,4]);
    // A.push(vec![5,6,7,8]);
    // A.push(vec![1,2,3,4]);
    // A.push(vec![5,6,7,8]);
    // let mut B = Vec::new();
    // B.push(vec![1,2,3,4]);
    // B.push(vec![5,6,7,8]);
    // B.push(vec![1,2,3,4]);
    // B.push(vec![5,6,7,8]);

    let (tl,tr,bl,br) = corners(&A);
    println!{"tl: {:?} \ntr: {:?} \nbl: {:?} \nbr: {:?}", tl, tr, bl, br};

    // let C = matrix_multiply(&mut A,&mut B);

    // println!("{:?}", C);
    // let args: Vec<String> = env::args().collect();
    // let size = args[1].parse::<usize>().unwrap();
    // let mut reps = args[2].parse::<usize>().unwrap();

    // let file_name = String::from("s".to_string() + &args[1] + &".txt".to_string());
    // let mut output = File::create(file_name).unwrap();

    // let mut rng = WyRand::new();
    // let mut arr = init_arr(size);
    // let mut avg: f32 = 0.0;
    // let mut iteration = 1;

    // while reps > 1 {
    //     let data: Data = quick_sort_rt(&mut arr);
    //     rng.shuffle(&mut arr);
    //     // let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
    //     // let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();
    //     let res = *data.dmd;

    //     avg += res;

    //     iteration += 1;
    //     reps -= 1;
    // }
    // avg /= iteration as f32;

    // // rng.shuffle(&mut arr);
    // // let data = quick_sort_rt(&mut arr);
    // // println!("{}", data.dmd);

    // write!(
    //     output,
    //     "arr_size: {}\nreps: {}\navg: {}\n",
    //     size, iteration, avg
    // )?;

    Ok(())
}
