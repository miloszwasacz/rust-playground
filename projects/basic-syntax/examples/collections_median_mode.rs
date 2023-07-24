use std::collections::HashMap;

fn main() {
    let values = [2, 1, 5, 3, 4, 4];
    let mut v = Vec::new();
    for val in values.iter() {
        v.push(*val);
    }

    v.sort();
    println!("Median = {}", median(&v));
    let mode = mode(&v);
    match mode {
        Some(x) => println!("Mode = {}", x),
        None => println!("Empty list"),
    }
}

fn median(v: &Vec<i32>) -> f32 {
    if v.len() % 2 == 0 {
        let i: usize = v.len() / 2;
        return ((v[i] + v[i - 1]) as f32) / 2.0;
    } else {
        return v[v.len() / 2] as f32;
    }
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    if v.len() == 0 {
        return None;
    }

    let mut max = v[0];
    let mut map = HashMap::new();
    for val in v.iter() {
        let i = map.entry(val).or_insert(0);
        *i += 1;
        if map[val] > map[&max] {
            max = *val;
        }
    }
    return Some(max);
}
