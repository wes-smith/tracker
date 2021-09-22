#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod instrumented_algorithms;
// mod lib;
// mod parse;
mod rttrace;



// use instrumented_algorithms::init_arr;
// use crate::rttrace::Data;
use instrumented_algorithms::{init,strassen, init_i32};
// use nanorand::{Rng, WyRand};

// use crate::rttrace::Data;
// use crate::rttrace::{init,trace};

use std::env;
use std::fs::File;
use std::io::prelude::*;



fn main() -> std::io::Result<()> {
    test_stras()?;
    // let mut a = Vec::new();
    // let mut b = Vec::new();
    // a.push(vec![1,2]);
    // a.push(vec![1,2]);
    // b.push(vec![1,2]);
    // b.push(vec![1,2]);

    // let (c, mmdata) = strassen(&mut a,&mut b);
    // println!("{:?}", c);
    Ok(())
}

fn test_stras() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();

    //let file_name = String::from("mm_s".to_string() + &args[1] + &".txt".to_string());
    let freq_file_name = String::from("stras_freq_s".to_string() + &args[1] + &".txt".to_string());
    //let mut output = File::create(file_name).unwrap();
    let mut freq_output = File::create(freq_file_name).unwrap();

    // let mut rng = WyRand::new();
    let (mut a, mut b) = init_i32(size as i32);

    // let (C, _mmdata) = multiply(&mut A,&mut B);
    // println!("{:?}\n{:?}\n{:?}", A,B,C);



    let (_c, mmdata) = strassen(&mut a,&mut b);
    let a_b_dmd = mmdata.a_b.dmd;
    let cc_dmd = mmdata.c.dmd;
    let temp_dmd = mmdata.temp.dmd;
    let total_dmd = *a_b_dmd + *cc_dmd + *temp_dmd;

    //println!("{:#?}", mmdata.a_b.freq_map);
    //println!("{:?}", _c);
    
    // write!(
    //     output,
    //     "mat_size: {}x{}\ntotal: {}\nA_B: {}\nC: {}\ntemp: {}\n",
    //     size, size, total_dmd, a_b_dmd, cc_dmd, temp_dmd
    // )?;
    
    write!(
        freq_output,
        "{:?}\n{:?}\n{:?}\n",
        mmdata.a_b.freq_map, mmdata.c.freq_map,mmdata.temp.freq_map
    )?;

    Ok(())
}

// fn test_mm(){
//     let args: Vec<String> = env::args().collect();
//     let size = args[1].parse::<usize>().unwrap();

//     //let file_name = String::from("mm_s".to_string() + &args[1] + &".txt".to_string());
//     let freq_file_name = String::from("mm_freq_s".to_string() + &args[1] + &".txt".to_string());
//     //let mut output = File::create(file_name).unwrap();
//     let mut freq_output = File::create(freq_file_name).unwrap();

//     // let mut rng = WyRand::new();
//     let (mut a, mut b) = init(size);

//     // let (C, _mmdata) = multiply(&mut A,&mut B);
//     // println!("{:?}\n{:?}\n{:?}", A,B,C);



//     let (_c, mmdata) = multiply(&mut a,&mut b);
//     let a_b_dmd = mmdata.a_b.dmd;
//     let cc_dmd = mmdata.c.dmd;
//     let temp_dmd = mmdata.temp.dmd;
//     let total_dmd = *a_b_dmd + *cc_dmd + *temp_dmd;

//     //println!("{:#?}", mmdata.a_b.freq_map);
//     // write!(
//     //     output,
//     //     "mat_size: {}x{}\ntotal: {}\nA_B: {}\nC: {}\ntemp: {}\n",
//     //     size, size, total_dmd, a_b_dmd, cc_dmd, temp_dmd
//     // )?;
//     write!(
//         freq_output,
//         "{:?}\n{:?}\n{:?}\n",
//         mmdata.a_b.freq_map, mmdata.c.freq_map,mmdata.temp.freq_map
//     )?;
// }


// fn test_qs() -> std::io::Result<()>{
//     let args: Vec<String> = env::args().collect();
//     let size = args[1].parse::<usize>().unwrap();
//     let mut reps = args[2].parse::<usize>().unwrap();

//     let file_name = String::from("s".to_string() + &args[1] + &".txt".to_string());
//     let mut output = File::create(file_name).unwrap();

//     let mut rng = WyRand::new();
//     let mut arr = init_ar(size);
//     let mut avg: f32 = 0.0;
//     let mut iteration = 1;

//     while reps > 1 {
//         let data: Data = qs(&mut arr);
//         rng.shuffle(&mut arr);
//         // let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
//         // let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();
//         let res = *data.dmd;

//         avg += res;

//         iteration += 1;
//         reps -= 1;
//     }
//     avg /= iteration as f32;

//     // rng.shuffle(&mut arr);
//     // let data = quick_sort_rt(&mut arr);
//     // println!("{}", data.dmd);

//     write!(
//         output,
//         "arr_size: {}\nreps: {}\navg: {}\n",
//         size, iteration, avg
//     )?;

//     Ok(())
// }
