// https://atcoder.jp/contests/abc086/tasks/arc089_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut now = 0;
    let mut nx = 0;
    let mut ny = 0;
    for _ in 0..N {
        input! {
            t: isize,
            x: isize,
            y: isize,
        }
        if t - now < (nx - x).abs() + (ny - y).abs() {
            println!("No");
            return;
        }
        now += (nx - x).abs() + (ny - y).abs();
        if (t - now) % 2 == 1 {
            println!("No");
            return;
        }
        now = t;
        nx = x;
        ny = y;
    }
    println!("Yes");
}