// https://atcoder.jp/contests/abc058/tasks/abc058_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        O: String,
        E: String,
    }
    let O = O.chars().collect::<Vec<char>>();
    let E = E.chars().collect::<Vec<char>>();
    let N = E.len();
    for i in 0..N {
        print!("{}", O[i]);
        print!("{}", E[i]);
    }
    if O.len() > N {
        print!("{}", O.last().unwrap());
    }
    println!();
}