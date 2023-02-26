// https://atcoder.jp/contests/abc215/tasks/abc215_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if S == "Hello,World!" {
        println!("AC");
    }
    else {
        println!("WA");
    }
}