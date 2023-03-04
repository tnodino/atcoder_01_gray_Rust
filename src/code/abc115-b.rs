// https://atcoder.jp/contests/abc115/tasks/abc115_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let sum = p.iter().sum::<usize>();
    println!("{}", sum - p.iter().max().unwrap() / 2);
}