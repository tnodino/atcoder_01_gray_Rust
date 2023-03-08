// https://atcoder.jp/contests/abc224/tasks/abc224_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        X.push(x);
        Y.push(y);
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                if ((X[i] - X[k]) * (Y[j] - Y[k]) - (X[j] - X[k]) * (Y[i] - Y[k])).abs() > 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}