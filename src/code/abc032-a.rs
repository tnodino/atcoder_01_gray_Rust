// https://atcoder.jp/contests/abc032/tasks/abc032_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        mut n: usize,
    }
    while n % a != 0 || n % b != 0 {
        n += 1;
    }
    println!("{}", n);
}