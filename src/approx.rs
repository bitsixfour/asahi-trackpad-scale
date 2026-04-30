// later
use std::collections::VecDeque;


pub struct Graph {
    pub time: i32,
    pub pressure: i32,
}

fn sort_graph(vec: VecDeque<(f64, f64)>) -> VecDeque<(f64, f64)>{
    let len: i32 = vec.len();
    let var: i32 =
    match len {
        11..i32::MAX => vec.truncate(len - 10),
        0..10 =>

    }

}
