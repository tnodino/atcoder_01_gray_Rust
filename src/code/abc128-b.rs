// https://atcoder.jp/contests/abc128/tasks/abc128_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for i in 0..N {
        input! {
            S: String,
            P: usize,
        }
        vec.push((S, P, i+1));
    }
    vec.sort_by(|a, b| if a.0 != b.0 {
        a.0.cmp(&b.0)
    }
    else {
        b.1.cmp(&a.1)
    });
    for i in 0..N {
        println!("{}", vec[i].2);
    }
}