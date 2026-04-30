// later
use std::collections::VecDeque;


pub struct Graph {
    pub time: i32,
    pub pressure: i32,
}


pub fn sort_graph(vec: &mut Vec<(f64, f64)>, pres: i32) -> &mut Vec<(f64,f64)> {
    let tuple: (f64, f64) = (vec.len() as f64, pres as f64);
    while vec.len() > 10 {
        vec.remove(0);
    }
    for i in 0..vec.len() {
          vec[i].0 = i as f64;
    }
    vec.push(tuple);
    vec
}
