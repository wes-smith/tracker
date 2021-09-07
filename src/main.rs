mod instrumented_algorithms;
mod lib;
mod parse;

use instrumented_algorithms::init_arr;
use instrumented_algorithms::quick_sort;
// use instrumented_algorithms::shuffle;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use nanorand::{Rng, WyRand};

fn main() -> std::io::Result<()> {

    let arr = vec!["a", "b", "c", "a", "b", "c"];
    lib::reuse_distance_eff(arr);

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
    //     quick_sort(&mut arr, "foo.log");
    //     rng.shuffle(&mut arr);
    //     let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
    //     let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();
    //     let res = lib::dmd_from_trace(trace_ref);

    //     avg += res;

    //     iteration += 1;
    //     reps -= 1;
    // }
    // avg /= iteration as f32;

    // write!(
    //     output,
    //     "arr_size: {}\nreps: {}\navg: {}\n",
    //     size, iteration, avg
    // )?;

    Ok(())
}
