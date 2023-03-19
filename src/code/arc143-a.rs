// https://atcoder.jp/contests/arc143/tasks/arc143_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut array = [A, B, C];
    array.sort();
    let ma = array[2];
    let mut cnt = 0;
    for i in 0..3 {
        cnt += ma - array[i];
    }
    if ma >= cnt {
        println!("{}", ma);
    }
    else {
        println!("-1");
    }
}