use std::time::Instant;

fn main() {
    let mut v: Vec<i64> = (0..1_000_000_000).rev().step_by(2).collect();
    //let mut v: Vec<i64> = (0..300).rev().collect();
    let now = Instant::now();
    //ips4o::sort(&mut v);
    v.sort_unstable();

    let elapsed = now.elapsed();
    println!("Elapsed: {:#?}", elapsed);

    //println!("Got: {:?}", v);
}