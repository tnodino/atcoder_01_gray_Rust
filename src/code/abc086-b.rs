// https://atcoder.jp/contests/abc086/tasks/abc086_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: String,
        b: String,
    }
    let v = format!("{}{}", a, b).parse::<usize>().unwrap();
    for i in 1..=v {
        if i * i == v {
            println!("Yes");
            return;
        }
    }
    println!("No");
}