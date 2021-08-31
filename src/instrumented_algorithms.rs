use nanorand::{Rng, WyRand};

pub fn quick_sort(arr: &mut Vec<i32>){
    let low = 0;
    let high = arr.len() as i32;
    quick_sort_helper(arr, low, high);
}

fn quick_sort_helper(arr: &mut Vec<i32>, low: i32, high: i32){
    if low < high{
        let pivot = partition(arr,low,high);
        quick_sort_helper(arr, low, pivot-1);
        quick_sort_helper(arr, pivot+1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32{
    let pivot = rng(low,high);
    let mut index = low - 1;
    let mut last = high;

    loop {
        index+=1;
        while arr[index as usize] < arr[pivot as usize] {
            index+=1;
        }
        last -= 1;
        while last >= 0 && arr[last as usize] > arr[pivot as usize]{
            last -= 1;
        }
        if index >= last{
            break;
        }
        else{
            arr.swap(index as usize, last as usize);
        }
    }
    arr.swap(index as usize, pivot as usize);
    index
}

fn rng(low: i32, high: i32) -> i32{
    let mut rng = WyRand::new();
    rng.generate_range(low..=high)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn smoke() {
        assert!(1 == 1);
    }

    #[test]
    fn sort() {
        let mut arr = vec![8,7,6,5,3,2,1];
        let mut arr2 = arr.clone();
        arr2.sort();
        quick_sort(&mut arr);
        assert_eq!(arr,arr2);
    }
}