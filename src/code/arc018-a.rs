// https://atcoder.jp/contests/arc018/tasks/arc018_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut Height: f64,
        BMI: f64,
    }
    Height /= 100.;
    println!("{}", BMI * Height * Height);
}