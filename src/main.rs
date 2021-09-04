mod lib;
mod instrumented_algorithms;
mod parse;

use instrumented_algorithms::quick_sort;
// use csv::ReaderBuilder;

fn main() {
    let trace: Vec<String> = parse::build_trace("foo.log").unwrap();
    let trace_ref: Vec<&str> = trace.iter().map(AsRef::as_ref).collect();

    let res = lib::dmd_from_trace(trace_ref);
    println!("{}", res)

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

// use csv::Reader;


// #[derive(Debug, Deserialize)]
// struct Record {
//     access_type: String,
//     id: String
// }

// fn test()-> std::io::Result<()> {

    
// }
