// https://atcoder.jp/contests/abc181/tasks/abc181_c

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
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                let x1 = x[i] - x[k];
                let x2 = x[j] - x[k];
                let y1 = y[i] - y[k];
                let y2 = y[j] - y[k];
                if x1 * y2 == x2 * y1 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}