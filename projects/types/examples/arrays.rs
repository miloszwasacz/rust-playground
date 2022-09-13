use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("slice first element: {}", slice[0]);
    println!("slice length: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("xs length = {}", xs.len());
    println!("xs memory usage: {}B", mem::size_of_val(&xs));

    println!("\nborrow whole array as a slice");
    analyze_slice(&xs);

    println!("\nborrow a section of an array as a slice");
    analyze_slice(&ys[1..4]);
    println!();

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("Element at {i} is {xval}"),
            None => println!("Index out of bounds! ({})", i),
        }
    }
}
