// https://atcoder.jp/contests/abc025/tasks/abc025_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: isize,
        B: isize,
    }
    let mut now = 0;
    for _ in 0..N {
        input! {
            s: String,
            d: isize,
        }
        let flg = match s.as_str() {
            "East" => 1,
            "West" => -1,
            _ => unreachable!()
        };
        if d < A {
            now += A * flg;
        }
        else if B < d {
            now += B * flg;
        }
        else {
            now += d * flg;
        }
    }
    if 0 < now {
        println!("East {}", now);
    }
    else if now < 0 {
        println!("West {}", now.abs());
    }
    else {
        println!("0");
    }
}