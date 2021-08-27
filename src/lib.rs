use std::collections::HashMap;

pub fn reuse_interval(trace: Vec<&str>) -> (Vec<usize>, HashMap<usize, f32>) {
    let mut map = HashMap::new();
    let mut res = Vec::new();
    let mut calc = Vec::new();

    for (i, val) in trace.iter().enumerate() {
        println!("index: {}  value: {}", i, val);
        if !map.contains_key(val) {
            map.insert(val, i);
            res.push(0);
        } else {
            let dist = i - map[val];
            res.push(dist);
            calc.push(dist);
            map.insert(val, i);
        }
    }

    let denom = calc.len();
    let mut freq_map = HashMap::new(); //value,frequency map
    for i in calc {
        if !freq_map.contains_key(&i) {
            freq_map.insert(i, 1);
        } else {
            freq_map.insert(i, freq_map[&i] + 1);
        }
    }

    let mut dist_map = HashMap::new();
    for (k, v) in freq_map.iter() {
        let freq: f32 = *v as f32 / denom as f32;
        dist_map.insert(k.clone(), freq);
    }

    return (res, dist_map);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn ri_cyclic() {
        let arr = vec!["a", "b", "c", "a", "b", "c"];
        let (res, dist) = reuse_interval(arr);

        assert_eq!(res, vec![0, 0, 0, 3, 3, 3]);

        let mut map = HashMap::new();
        map.insert(3, 1.0);

        assert_eq!(dist, map);
    }

    #[test]
    fn ri_sawtooth() {
        let arr = vec!["a", "b", "c", "c", "b", "a"];
        let (res, dist) = reuse_interval(arr);

        assert_eq!(res, vec![0, 0, 0, 1, 3, 5]);

        let mut map = HashMap::new();
        map.insert(1, 0.33333334);
        map.insert(3, 0.33333334);
        map.insert(5, 0.33333334);

        assert_eq!(dist, map);
    }
}
