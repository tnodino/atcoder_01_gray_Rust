// https://atcoder.jp/contests/abc306/tasks/abc306_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N*3],
    }
    let mut idx = vec![Vec::new(); N+1];
    for i in 0..N*3 {
        idx[A[i]].push(i);
    }
    let mut vec = Vec::new();
    for i in 1..=N {
        vec.push((idx[i][1], i));
    }
    vec.sort();
    println!("{}", vec.iter().map(|&x| x.1.to_string()).collect::<Vec<String>>().join(" "));
}