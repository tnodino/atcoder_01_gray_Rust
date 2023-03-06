// https://atcoder.jp/contests/abc246/tasks/abc246_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
    }
    let x4 = match (x1 == x2, x1 == x3) {
        (true, _) => x3,
        (_, true) => x2,
        _ => x1,
    };
    let y4 = match (y1 == y2, y1 == y3) {
        (true, _) => y3,
        (_, true) => y2,
        _ => y1,
    };
    println!("{} {}", x4, y4);
}