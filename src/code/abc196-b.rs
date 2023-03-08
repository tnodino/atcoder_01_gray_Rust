// https://atcoder.jp/contests/abc196/tasks/abc196_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
    }
    let idx = X.find('.');
    if idx == None {
        println!("{}", X);
    }
    else {
        println!("{}", &X[..idx.unwrap()]);
    }
}