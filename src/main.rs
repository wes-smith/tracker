mod lib;
mod instrumented_algorithms;
mod parse;

use instrumented_algorithms::quick_sort;
use instrumented_algorithms::init_arr;
// use instrumented_algorithms::shuffle;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use nanorand::{Rng, WyRand};

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    let mut reps = args[2].parse::<usize>().unwrap();

    let mut output = File::create("output.txt").unwrap();

    let mut rng = WyRand::new();
    let mut arr = init_arr(size);
    let mut avg: f32 = 0.0;
    let mut iteration = 1;
    while reps > 1 {
        quick_sort(&mut arr, "smoke");
        rng.shuffle(&mut arr);
        let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
        let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();
        let res = lib::dmd_from_trace(trace_ref);

        avg = (avg+res)/iteration as f32;

        iteration += 1;
        reps -= 1;
    }

    write!(output, "arr_size: {}\nreps: {}\navg: {}\n", size, iteration, avg)?;

    Ok(())
    

    // let mut iter = parse::file_to_iter("foo.log").unwrap();

    // while let Some(record) = iter.next() {
    //     println!("{:?}", record);
    // }

    // let arr = vec!["a", "b", "c", "a", "b", "c"];
    // let arr = vec![
    //     "a", "b", "a", "b", "a", "b", "c", "c", "c", "c", "c", "c", "a",
    // ];
    // let _map = lib::reuse_interval(arr);
    // let arr = vec!["a","b","b","b","c","b","b","a"];
    // let (_freq,_res) = lib::reuse_distance(arr);
    // let res = lib::dmd(_freq);
    // println!("{}", res);


    // let mut arr = vec![8,7,6,5];
    // quick_sort(&mut arr);
}
