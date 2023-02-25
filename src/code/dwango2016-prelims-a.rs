// https://atcoder.jp/contests/dwango2016-prelims/tasks/dwango2016qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N / 25);
}