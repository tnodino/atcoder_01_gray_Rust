// https://atcoder.jp/contests/abc033/tasks/abc033_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = 0;
    let mut vec = vec![];
    for _ in 0..N {
        input! {
            S: String,
            P: usize,
        }
        cnt += P;
        vec.push((S, P));
    }
    cnt = cnt / 2;
    for i in 0..N {
        if vec[i].1 > cnt {
            println!("{}", vec[i].0);
            return;
        }
    }
    println!("atcoder");
}