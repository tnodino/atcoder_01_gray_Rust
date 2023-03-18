// https://atcoder.jp/contests/abc203/tasks/abc203_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
        mut AB: [(usize, usize); N],
    }
    AB.sort();
    let mut pos = 0;
    for i in 0..N {
        if AB[i].0 - pos > K {
            println!("{}", pos + K);
            return;
        }
        K -= AB[i].0 - pos;
        K += AB[i].1;
        pos = AB[i].0;
    }
    println!("{}", pos + K);
}