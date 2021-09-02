mod lib;
mod instrumented_algorithms;

use instrumented_algorithms::quick_sort;

fn main() {
    // let arr = vec!["a", "b", "c", "a", "b", "c"];
    // let arr = vec![
    //     "a", "b", "a", "b", "a", "b", "c", "c", "c", "c", "c", "c", "a",
    // ];
    // let _map = lib::reuse_interval(arr);
    // let arr = vec!["a","b","b","b","c","b","b","a"];
    // let _res = lib::reuse_distance(arr);
    // println!("{:?}", _res);


    let mut arr = vec![8,7,6,5];
    quick_sort(&mut arr);
    println!("{:?}",arr);
}
