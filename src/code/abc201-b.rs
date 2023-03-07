// https://atcoder.jp/contests/abc201/tasks/abc201_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            S: String,
            T: usize,
        }
        vec.push((T, S));
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{}", vec[1].1);
}