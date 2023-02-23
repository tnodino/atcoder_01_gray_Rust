// https://atcoder.jp/contests/arc012/tasks/arc012_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        day: String,
    }
    println!("{}" match day.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        "Saturday" => 0,
        "Sunday" => 0,
        _ => unreachable!()
    });
}