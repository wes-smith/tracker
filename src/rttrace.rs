use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
 	stack: Box<Vec<String>>,
	freq_map: Box<HashMap<usize,usize>>,
	pub dmd: Box<f32>,
}

pub fn init() -> Data {
	let mut stack = Box::new(Vec::new());
	let mut freq_map: Box<HashMap<usize,usize>> = Box::new(HashMap::new());
	let mut dmd = Box::new(0.0);

	let mut d =  Data {
		stack,
		freq_map,
		dmd,
	};
	d
}

pub fn trace(val: String, data: &mut Data){
	// let mut data: Data = &*d;
	
    if data.stack.contains(&val){ //resuse
        let position = data.stack.iter().position(|x| *x == val).unwrap();  //get position in stack
        if position == data.stack.len()-1{ //top of stack
            let freq = data.freq_map.entry(1).or_insert(0);
            *freq += 1;
        }
        else{
            let item = data.stack.remove(position);    //remove element and place at top
            data.stack.push(item);
            let temp_dist = data.stack.len()-position;
            
            let freq = data.freq_map.entry(temp_dist).or_insert(0);
            *freq += 1;

            *data.dmd += *freq as f32 * (temp_dist as f32).sqrt();
        }
    }
    else{
        data.stack.push(val);
    }
}