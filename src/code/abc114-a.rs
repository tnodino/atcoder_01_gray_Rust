// https://atcoder.jp/contests/abc114/tasks/abc114_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let array = [3, 5, 7];
    if array.contains(&X) {
        println!("YES");
    }
    else {
        println!("NO");
    }
}