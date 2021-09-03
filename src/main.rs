mod lib;
mod instrumented_algorithms;

use instrumented_algorithms::quick_sort;
use csv;
use std::error::Error;
use std::io;

fn main() {


    // let arr = vec!["a", "b", "c", "a", "b", "c"];
    // let arr = vec![
    //     "a", "b", "a", "b", "a", "b", "c", "c", "c", "c", "c", "c", "a",
    // ];
    // let _map = lib::reuse_interval(arr);
    // let arr = vec!["a","b","b","b","c","b","b","a"];
    // let (_freq,_res) = lib::reuse_distance(arr);
    // let res = lib::dmd(_freq);
    // println!("{}", res);


    let mut arr = vec![8,7,6,5];
    quick_sort(&mut arr);
    
    let _res = test();
}

fn test() -> Result<(), Box<dyn Error>> {
    

    let mut rdr = csv::Reader::from_reader(file.as_bytes());
    for result in rdr.deserialize(){
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
