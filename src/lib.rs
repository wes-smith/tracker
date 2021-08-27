use std::collections::HashMap;

pub fn reuse_interval(trace: Vec<&str>) -> Vec<usize>{
    let mut map = HashMap::new();
    let mut res = Vec::new();

    for (i,val) in trace.iter().enumerate(){
        println!("index: {}  value: {}", i, val);
        if !map.contains_key(val){
            map.insert(val,i);
            res.push(0);
        }
        else{
            let dist = i - map[val];
            res.push(dist);
            map.insert(val,i);
        }
    }
    return res;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke(){
        assert!(1==1);
    }

    #[test]
    fn ri_cyclic(){
        let arr = vec!["a","b","c","a","b","c"];
        let res = reuse_interval(arr);
        assert_eq!(res, vec![0,0,0,3,3,3]);
    }

    #[test]
    fn ri_sawtooth(){
        let arr = vec!["a","b","c","c","b","a"];
        let res = reuse_interval(arr);
        assert_eq!(res, vec![0,0,0,1,3,5]);
    }
}