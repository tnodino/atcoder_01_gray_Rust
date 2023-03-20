// https://atcoder.jp/contests/abc294/tasks/abc294_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    println!("{}", A.iter().filter(|&x| x % 2 == 0).map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}