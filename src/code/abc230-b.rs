// https://atcoder.jp/contests/abc230/tasks/abc230_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let T = "oxx".repeat(10);
    if T.contains(&S) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}