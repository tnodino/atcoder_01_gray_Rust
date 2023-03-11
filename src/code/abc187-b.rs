// https://atcoder.jp/contests/abc187/tasks/abc187_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..N {
        input! {
            a: isize,
            b: isize,
        }
        x.push(a);
        y.push(b);
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            if (y[i] - y[j]).abs() <= (x[i] - x[j]).abs() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}