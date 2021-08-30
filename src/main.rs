mod lib;

fn main() {
    // let arr = vec!["a", "b", "c", "a", "b", "c"];
    // let arr = vec![
    //     "a", "b", "a", "b", "a", "b", "c", "c", "c", "c", "c", "c", "a",
    // ];
    // let _map = lib::reuse_interval(arr);
    let arr = vec!["a","b","b","b","c","b","b","a"];
    let _res = lib::reuse_distance(arr);
    println!("{:?}", _res);
}
