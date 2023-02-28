// https://atcoder.jp/contests/abc090/tasks/abc090_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    for i in 0..3 {
        input! {
            c: String,
        }
        print!("{}", &c[i..i+1]);
    }
    println!();
}