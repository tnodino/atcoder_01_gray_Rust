// https://atcoder.jp/contests/abc309/tasks/abc309_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut vec = vec![(0, 0)];
    let mut s = 0;
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        s += b;
        vec.push((a, b));
    }
    vec.sort();
    for i in 0..=N {
        s -= vec[i].1;
        if s <= K {
            println!("{}", vec[i].0 + 1);
            return;
        }
    }
}