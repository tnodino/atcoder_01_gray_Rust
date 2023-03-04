// https://atcoder.jp/contests/abc216/tasks/abc216_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut sp = S.trim().split('.');
    let X = sp.next().unwrap();
    let X = X.parse::<usize>().unwrap();
    let Y = sp.next().unwrap();
    let Y = Y.parse::<usize>().unwrap();
    if Y <= 2 {
        println!("{}-", X);
    }
    else if Y <= 6 {
        println!("{}", X);
    }
    else {
        println!("{}+", X);
    }

}