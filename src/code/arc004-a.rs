// https://atcoder.jp/contests/arc004/tasks/arc004_1

use proconio::input;
use proconio::fastout;
use libm::hypot;

fn max(x: f64, y: f64) -> f64 {
    return match x >= y {
        true => x,
        false => y,
    };
}

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
            (i1, i2): (f64, f64),
        }
        x.push(i1);
        y.push(i2);
    }
    let mut ans = 0.;
    for i in 0..N {
        for j in i+1..N {
            let dist = hypot((x[i] - x[j]).abs(), (y[i] - y[j]).abs());
            ans = max(ans, dist);
        }
    }
    println!("{}", ans);
}